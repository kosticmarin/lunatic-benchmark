use std::time::Instant;

use clap::Parser;
use lunatic::{spawn, Mailbox, Process};
use lunatic_bench::{parse_byte_size, ClientStats, TransferResult};
use rand::{distributions::Alphanumeric, seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Clone, Copy, Serialize, Deserialize)]
#[clap(name = "bulk")]
pub struct Opt {
    /// Number of clients on a local node to spawn
    #[clap(long, default_value = "2")]
    pub clients: i32,
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
fn main(m: Mailbox<i32>) {
    let opt = Opt::parse();
    let nodes = lunatic::distributed::nodes();

    let this = m.this();

    let mut processes = vec![];
    for i in 0..opt.clients {
        let nodes = nodes.clone();
        let parent = this.clone();
        let args = Args {
            client_id: i,
            parent,
            nodes,
            opt,
        };
        let p = spawn!(|args, mailbox: Mailbox<String>| spawn_and_ping_remote(args, mailbox));
        processes.push(p);
    }

    for _ in &processes {
        m.receive();
    }

    for p in processes {
        p.send("".to_string());
        m.receive();
    }
}

#[derive(Serialize, Deserialize)]
struct Args {
    client_id: i32,
    parent: Process<i32>,
    nodes: Vec<u64>,
    opt: Opt,
}

// local
fn spawn_and_ping_remote(args: Args, mailbox: Mailbox<String>) {
    let this = mailbox.this();
    let mut stats = ClientStats::default();
    let start = Instant::now();
    let mut rng = rand::thread_rng();
    args.nodes.choose(&mut rng).into_iter().for_each(|node| {
        let remote = Process::spawn_node(*node, this.clone(), pong);
        for _ in 0..=args.opt.requests {
            let data: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(args.opt.message_size as usize)
                .map(char::from)
                .collect();
            let start = Instant::now();
            remote.send(data);
            let upload_result = TransferResult::new(start.elapsed(), args.opt.message_size as u64);

            let start = Instant::now();
            let _ = mailbox.receive();
            let download_result =
                TransferResult::new(start.elapsed(), args.opt.message_size as u64);

            stats.upload_stats.stream_finished(upload_result);
            stats.download_stats.stream_finished(download_result);
        }
    });
    stats.download_stats.total_duration = start.elapsed();
    stats.upload_stats.total_duration = start.elapsed();
    args.parent.send(0);
    mailbox.receive();
    stats.print(args.client_id as usize);
    args.parent.send(1);
}

// remote
fn pong(parent: Process<String>, mailbox: Mailbox<String>) {
    loop {
        let v = mailbox.receive();
        parent.send(v);
    }
}
