pub mod actions;
pub mod cli_opts;
pub mod listener;
pub mod node;
pub mod sync_job;
pub mod whisper_job;
use crate::node::Node;
use once_cell::sync::OnceCell;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

static NODE: OnceCell<Arc<Mutex<Node>>> = OnceCell::new();

pub fn node() -> &'static Arc<Mutex<Node>> {
    NODE.get().unwrap()
}

pub fn create_node(address: SocketAddr) {
    NODE.set(Arc::new(Mutex::new(Node::new(address)))).unwrap()
}
