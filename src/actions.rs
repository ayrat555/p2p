use isahc::prelude::*;
use std::net::SocketAddr;
use std::time::Duration;

pub mod add_peer;
pub mod broadcast_peer;
pub mod connect;
pub mod ping;
pub mod return_peers;
pub mod send_whisper;
pub mod sync_peers;
pub mod whisper;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub msg: String,
}

pub fn client(address: &SocketAddr) -> HttpClient {
    HttpClient::builder()
        .timeout(Duration::from_secs(5))
        .default_header("NODE", address.to_string())
        .build()
        .unwrap()
}
