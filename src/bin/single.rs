use std::time::Instant;

use clap::Parser;
use lunatic::{Mailbox, Process};
use lunatic_bench::{parse_byte_size, ClientStats, TransferResult};
use rand::{distributions::Alphanumeric, Rng};

#[derive(Parser, Debug, Clone, Copy)]
#[clap(name = "bulk")]
pub struct Opt {
    /// Number of bytes to transmit from node to node
    ///
    /// This can use SI prefixes for sizes. E.g. 1M will transfer 1MiB, 10GiB
    /// will transfer 10GiB.
    #[clap(long, default_value = "1k", parse(try_from_str = parse_byte_size))]
    pub message_size: u64,
    /// Number of requests to make from node to node
    #[clap(long, default_value = "1000")]
    pub requests: u64,
}

#[lunatic::main]
fn main(mailbox: Mailbox<String>) {
    let opt = Opt::parse();

    let nodes = lunatic::distributed::nodes();

    let this = mailbox.this();
    let remote = Process::spawn_node(nodes[0], this, hello);

    let mut stats = ClientStats::default();
    let start = Instant::now();
    for _ in 0..opt.requests {
        let data: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(opt.message_size as usize)
            .map(char::from)
            .collect();
        let start = Instant::now();
        remote.send(data);
        let upload_result = TransferResult::new(start.elapsed(), opt.message_size);

        let start = Instant::now();
        let _ = mailbox.receive();
        let download_result = TransferResult::new(start.elapsed(), opt.message_size);

        stats.upload_stats.stream_finished(upload_result);
        stats.download_stats.stream_finished(download_result);
    }
    stats.upload_stats.total_duration = start.elapsed();
    stats.download_stats.total_duration = start.elapsed();
    stats.print(0);
}

fn hello(parent: Process<String>, mailbox: Mailbox<String>) {
    loop {
        let v = mailbox.receive();
        parent.send(v);
    }
}
