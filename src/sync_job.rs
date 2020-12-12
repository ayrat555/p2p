use crate::actions::sync_peers::call as sync_peers;
use crate::node;
use tokio::time;

pub async fn sync_loop() {
    let mut interval = time::interval(std::time::Duration::from_secs(10));

    loop {
        interval.tick().await;

        sync();
    }
}

fn sync() {
    let mut node = node().lock().unwrap();

    log::debug!("Started syncing");

    if let Err(error) = sync_peers(&mut node) {
        log::error!("Failed to sync peers {:?}", error);
    }
}
