use crate::actions::send_whisper::call as send_whisper;
use crate::node::Node;
use std::sync::{Arc, Mutex};
use tokio::time;

pub async fn whisper_loop(node: Arc<Mutex<Node>>, period: u64) {
    let mut interval = time::interval(std::time::Duration::from_secs(period));

    loop {
        interval.tick().await;

        whisper(node.clone());
    }
}

fn whisper(node: Arc<Mutex<Node>>) {
    let mut node = node.lock().unwrap();

    log::debug!("Started whispering");

    if let Err(error) = send_whisper(&mut node) {
        log::error!("Failed to send whisper {:?}", error);
    }
}
