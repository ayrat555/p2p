use crate::actions::send_whisper::call as send_whisper;
use crate::node;
use tokio::time;

pub async fn whisper_loop(period: u64) {
    let mut interval = time::interval(std::time::Duration::from_secs(period));

    loop {
        interval.tick().await;

        whisper();
    }
}

fn whisper() {
    let mut node = node().lock().unwrap();

    log::debug!("Started whispering");

    if let Err(error) = send_whisper(&mut node) {
        log::error!("Failed to send whisper {:?}", error);
    }
}
