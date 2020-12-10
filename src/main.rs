use clap::Clap;
use p2p::cli_opts::CliOpts;
use p2p::listener::Listener;
use p2p::node::Node;

fn main() {
    let opts = CliOpts::parse();
    let listener = create_listener(opts);

    listener.start();

    // println!("Period Value for config: {}", opts.period);
    // println!("Port Value for config: {}", opts.port);
    // println!("Connect Value for config: {:?}", opts.connect);
}

fn create_listener(opts: CliOpts) -> Listener {
    let node = create_node(opts);

    Listener::new(node)
}

fn create_node(opts: CliOpts) -> Node {
    let address = format!("{}:{}", "127.0.0.1", opts.port);

    Node::new(address)
}
