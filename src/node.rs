use std::net::SocketAddr;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub address: SocketAddr,
    pub peers: Vec<SocketAddr>,
}

impl Node {
    pub fn new(address: SocketAddr) -> Self {
        let peers = vec![];

        Node { address, peers }
    }

    pub fn add_peer(&mut self, address: SocketAddr) {
        self.peers.push(address);
    }

    pub fn remove_peer(&mut self, address: &SocketAddr) {
        if let Some(pos) = self.peers.iter().position(|x| *x == *address) {
            self.peers.remove(pos);
        }
    }

    pub fn peer_exists(&self, address: &SocketAddr) -> bool {
        *address == self.address || self.peers.contains(address)
    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    use std::net::SocketAddr;

    #[test]
    fn new_creates_new_node() {
        let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();

        let node = Node::new(address.clone());

        let expected_node = Node {
            address: address,
            peers: vec![],
        };
        assert_eq!(expected_node, node)
    }

    #[test]
    fn add_peer_adds_new_peer() {
        let node_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let peer_address: SocketAddr = "127.0.0.1:8081".parse().unwrap();
        let mut node = Node::new(node_address.clone());

        node.add_peer(peer_address.clone());

        let expected_node = Node {
            address: node_address,
            peers: vec![peer_address],
        };
        assert_eq!(expected_node, node)
    }

    #[test]
    fn peer_exists_returns_true_if_node_address_is_passed() {
        let node_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let node = Node::new(node_address.clone());

        assert!(node.peer_exists(&node_address));
    }

    #[test]
    fn peer_exists_returns_true_if_peer_exists() {
        let node_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let peer_address: SocketAddr = "127.0.0.1:8081".parse().unwrap();
        let mut node = Node::new(node_address.clone());
        node.add_peer(peer_address.clone());

        assert!(node.peer_exists(&peer_address));
    }

    #[test]
    fn peer_exists_returns_false_if_peer_does_not_exist() {
        let node_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let peer_address: SocketAddr = "127.0.0.1:8081".parse().unwrap();
        let node = Node::new(node_address.clone());

        assert!(!node.peer_exists(&peer_address));
    }
}
