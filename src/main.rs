use crate::node::Node;
use clap::Clap;
use p2p::actions::connect::call as connect;
use p2p::cli_opts::CliOpts;
use p2p::create_node;
use p2p::node;
use p2p::server;
use p2p::sync_job;
use p2p::whisper_job;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    env_logger::init();

    let opts = CliOpts::parse();

    let node = create_new_node(&opts);
    let address = node.address.clone();
    let mutex_node = Arc::new(Mutex::new(node));

    tokio::spawn(server::start_server(mutex_node.clone(), address));
    tokio::spawn(sync_job::sync_loop(mutex_node.clone()));

    maybe_connect_to_bootnode(mutex_node.clone(), &opts).await;

    whisper_job::whisper_loop(mutex_node.clone(), opts.period).await;
}

fn create_new_node(opts: &CliOpts) -> Node {
    let address: SocketAddr = format!("{}:{}", "127.0.0.1", opts.port)
        .parse()
        .expect("Node address is not valid");

    Node::new(address)
}

async fn maybe_connect_to_bootnode(node: Arc<Mutex<Node>>, opts: &CliOpts) {
    match &opts.connect {
        Some(address_str) => {
            time::delay_for(Duration::from_secs(5)).await;

            let address: SocketAddr = address_str.parse().expect("Bootnode address is not valid");
            let mut node = node.lock().unwrap();
            if let Err(error) = connect(&mut node, &address) {
                log::error!("Failed to connect to bootnode {:?}", error);
            }
        }
        None => (),
    }
}
