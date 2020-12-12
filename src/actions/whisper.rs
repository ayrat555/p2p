use std::net::SocketAddr;

fn call(message: &str, address: &SocketAddr) {
    log::info!("Received {} from {}", message, address.to_string());
}
