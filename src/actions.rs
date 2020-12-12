use isahc::prelude::*;
use std::time::Duration;

mod add_peer;
mod broadcast_peer;
mod ping;
mod return_peers;
mod sync_peers;

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
