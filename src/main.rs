use clap::Clap;
use once_cell::sync::OnceCell;
use p2p::cli_opts::CliOpts;
use p2p::listener::Listener;
use p2p::node::Node;
use std::net::SocketAddr;

static INSTANCE: OnceCell<Listener> = OnceCell::new();

#[tokio::main]
async fn main() {
    let opts = CliOpts::parse();

    if let Err(error) = listener(opts).start_server().await {
        log::error!("Failed to start server {:?}", error);
    }
}

fn listener(opts: CliOpts) -> &'static Listener {
    INSTANCE.get_or_init(|| Listener::new(create_node(opts)))
}

fn create_node(opts: CliOpts) -> Node {
    let address: SocketAddr = format!("{}:{}", "127.0.0.1", opts.port).parse().unwrap();

    Node::new(address)
}
