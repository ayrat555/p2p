use clap::Clap;
use p2p::actions::connect::call as connect;
use p2p::cli_opts::CliOpts;
use p2p::create_node;
use p2p::node;
use p2p::sync_job;
use p2p::whisper_job;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    env_logger::init();

    let opts = CliOpts::parse();

    create_new_node(&opts);
    maybe_connect_to_bootnode(&opts);

    tokio::spawn(sync_job::sync_loop());
    whisper_job::whisper_loop(opts.period).await
}

fn create_new_node(opts: &CliOpts) {
    let address: SocketAddr = format!("{}:{}", "127.0.0.1", opts.port)
        .parse()
        .expect("Node address is not valid");

    create_node(address);
}

fn maybe_connect_to_bootnode(opts: &CliOpts) {
    match &opts.connect {
        Some(address_str) => {
            let address: SocketAddr = address_str.parse().expect("Bootnode address is not valid");
            let mut node = node().lock().unwrap();
            if let Err(error) = connect(&mut node, &address) {
                log::error!("Failed to connect to bootnode {:?}", error);
            }
        }
        None => (),
    }
}
