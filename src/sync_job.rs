use crate::actions::sync_peers::call as sync_peers;
use crate::node;
use crate::node::Node;
use std::sync::{Arc, Mutex};
use tokio::time;

pub async fn sync_loop(node: Arc<Mutex<Node>>) {
    let mut interval = time::interval(std::time::Duration::from_secs(10));

    loop {
        interval.tick().await;

        sync(node.clone());
    }
}

fn sync(node: Arc<Mutex<Node>>) {
    let mut node = node.lock().unwrap();

    log::debug!("Started syncing");

    if let Err(error) = sync_peers(&mut node) {
        log::error!("Failed to sync peers {:?}", error);
    }
}
