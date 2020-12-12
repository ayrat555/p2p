use std::net::SocketAddr;

pub fn call(message: &str, address: &SocketAddr) {
    log::info!("Received {} from {}", message, address.to_string());
}
