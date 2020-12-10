#[derive(Debug, PartialEq)]
pub struct Node {
    address: String,
    peers: Vec<String>,
}

impl Node {
    // TODO: add validation
    pub fn new(address: String) -> Self {
        let peers = vec![];

        Node { address, peers }
    }

    // TODO: validate peer
    pub fn add_peer(&mut self, address: String) {
        self.peers.push(address);
    }

    pub fn peer_exists(&self, address: String) -> bool {
        address == self.address || self.peers.contains(&address)
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Node;

    #[test]
    fn new_creates_new_node() {
        let address = "127.0.0.1:8080".to_string();

        let node = Node::new(address.clone());

        let expected_node = Node {
            address: address,
            peers: vec![],
        };
        assert_eq!(expected_node, node)
    }

    #[test]
    fn add_peer_adds_new_peer() {
        let node_address = "127.0.0.1:8080".to_string();
        let peer_address = "127.0.0.1:8081".to_string();
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
        let node_address = "127.0.0.1:8080".to_string();
        let node = Node::new(node_address.clone());

        assert!(node.peer_exists(node_address));
    }

    #[test]
    fn peer_exists_returns_true_if_peer_exists() {
        let node_address = "127.0.0.1:8080".to_string();
        let peer_address = "127.0.0.1:8081".to_string();
        let mut node = Node::new(node_address.clone());
        node.add_peer(peer_address.clone());

        assert!(node.peer_exists(peer_address));
    }

    #[test]
    fn peer_exists_returns_false_if_peer_does_not_exist() {
        let node_address = "127.0.0.1:8080".to_string();
        let peer_address = "127.0.0.1:8081".to_string();
        let node = Node::new(node_address.clone());

        assert!(!node.peer_exists(peer_address));
    }
}
