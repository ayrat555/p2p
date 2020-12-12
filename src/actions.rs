use isahc::prelude::*;
use std::time::Duration;

pub mod add_peer;
pub mod broadcast_peer;
pub mod ping;
pub mod return_peers;
pub mod send_whisper;
pub mod sync_peers;
pub mod whisper;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub msg: String,
}

pub fn client() -> HttpClient {
    HttpClient::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}
