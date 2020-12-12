use clap::Clap;
use once_cell::sync::OnceCell;
use p2p::cli_opts::CliOpts;
use p2p::create_node;
use p2p::listener::Listener;
use p2p::node::Node;
use p2p::sync_job;
use p2p::whisper_job;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    env_logger::init();

    let opts = CliOpts::parse();
    create_new_node(&opts);

    tokio::spawn(sync_job::sync_loop());
    whisper_job::whisper_loop(opts.period).await
}

fn create_new_node(opts: &CliOpts) {
    let address: SocketAddr = format!("{}:{}", "127.0.0.1", opts.port).parse().unwrap();

    create_node(address);
}
