use std::time::Instant;

use clap::Parser;
use lunatic::{Mailbox, Process};
use lunatic_bench::{
    duration_from_epochs, get_epoch_secs, parse_byte_size, ClientStats, TransferResult,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Clone, Copy)]
#[clap(name = "bulk")]
pub struct Opt {
    /// Number of bytes to transmit from node to node
    ///
    /// This can use SI prefixes for sizes. E.g. 1M will transfer 1MiB, 10GiB
    /// will transfer 10GiB.
    #[clap(long, default_value = "250k", parse(try_from_str = parse_byte_size))]
    pub message_size: u64,
    /// Number of requests to make from node to node
    #[clap(long, default_value = "10")]
    pub requests: u64,
}

#[derive(Serialize, Deserialize)]
enum Message {
    String(String),
    Empty,
}

#[lunatic::main]
fn main(mailbox: Mailbox<Message>) {
    let opt = Opt::parse();

    let nodes = lunatic::distributed::nodes();

    let this = mailbox.this();
    let remote = Process::spawn_node(nodes[0], this, hello);

    let mut stats = ClientStats::default();
    let start = Instant::now();

    for i in 0..opt.requests {
        println!("MAIN sending msg {i}");
        let data: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(opt.message_size as usize)
            .map(char::from)
            .collect();
        let start = get_epoch_secs();
        remote.send(Message::String(data.clone()));
        let end = match mailbox.receive() {
            Message::Empty => Some(get_epoch_secs()),
            _ => None,
        };
        let upload_result = TransferResult::new(
            duration_from_epochs(start, end.expect("Invalid order of messages")),
            opt.message_size,
        );

        let start = match mailbox.receive() {
            Message::Empty => Some(get_epoch_secs()),
            _ => None,
        };
        let msg = mailbox.receive();
        if let Message::String(s) = msg {
            assert_eq!(s, data);
        }
        let end = get_epoch_secs();
        let download_result = TransferResult::new(
            duration_from_epochs(start.expect("Invalid order of messages"), end),
            opt.message_size,
        );

        stats.upload_stats.stream_finished(upload_result);
        stats.download_stats.stream_finished(download_result);
    }
    stats.upload_stats.total_duration = start.elapsed();
    stats.download_stats.total_duration = start.elapsed();
    stats.print(0);
}

fn hello(parent: Process<Message>, mailbox: Mailbox<Message>) {
    loop {
        let v = mailbox.receive();
        println!("REMOTE recv msg");
        parent.send(Message::Empty);
        parent.send(Message::Empty);
        parent.send(v);
    }
}
