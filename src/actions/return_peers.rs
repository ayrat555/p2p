use crate::node::Node;

pub fn call(node: Node) -> String {
    node.peers
        .into_iter()
        .map(|peer| peer.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::call;
    use crate::node::Node;
    use std::net::SocketAddr;

    #[test]
    fn call_returns_peers_string() {
        let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let peer_address1: SocketAddr = "127.0.0.1:8081".parse().unwrap();
        let peer_address2: SocketAddr = "127.0.0.1:8082".parse().unwrap();
        let node = Node {
            address: address,
            peers: vec![peer_address1.clone(), peer_address2.clone()],
        };

        let result = call(node);

        assert_eq!("127.0.0.1:8081,127.0.0.1:8082".to_string(), result);
    }
}
