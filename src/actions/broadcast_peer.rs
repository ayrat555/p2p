use crate::actions;
use crate::actions::add_peer::call as add_peer;
use crate::actions::Error;
use crate::node::Node;
use std::net::SocketAddr;

pub fn call(node: &mut Node, address: &SocketAddr) -> Result<(), Error> {
    let client = actions::client();
    let mut nodes_to_remove: Vec<SocketAddr> = vec![];

    for peer in &node.peers {
        let action_path = format!("http://{}/add_peer", peer.to_string());

        match client.post(action_path, address.to_string()) {
            Ok(response) => {
                if response.status() != 200 {
                    nodes_to_remove.push(peer.clone())
                }
            }

            Err(_err) => nodes_to_remove.push(peer.clone()),
        }
    }

    for peer in &nodes_to_remove {
        node.remove_peer(peer);
    }

    add_peer(node, &address)
}

#[cfg(test)]
mod tests {
    use super::call;
    use crate::node::Node;
    use httpmock::Method::GET;
    use httpmock::Method::POST;
    use httpmock::MockServer;
    use std::net::SocketAddr;

    #[test]
    fn call_returns_success() {
        let new_peer_server = MockServer::start();
        let mock1 = new_peer_server.mock(|when, then| {
            when.method(GET).path("/ping");
            then.status(200)
                .header("Content-Type", "text/html")
                .body("pong");
        });

        let old_peer_server = MockServer::start();
        let mock2 = old_peer_server.mock(|when, then| {
            when.method(POST).path("/add_peer");
            then.status(200);
        });

        let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let mut node = Node {
            address: address,
            peers: vec![old_peer_server.address().clone()],
        };

        let result = call(&mut node, new_peer_server.address());

        assert_eq!(Ok(()), result);
        mock1.assert();
        mock2.assert();

        let expected_result: Vec<SocketAddr> = vec![
            old_peer_server.address().clone(),
            new_peer_server.address().clone(),
        ];
        assert_eq!(expected_result, node.peers);
    }

    #[test]
    fn call_removes_peer_if_fails_to_respond() {
        let new_peer_server = MockServer::start();
        let mock = new_peer_server.mock(|when, then| {
            when.method(GET).path("/ping");
            then.status(200)
                .header("Content-Type", "text/html")
                .body("pong");
        });

        let old_peer_address: SocketAddr = "127.0.0.2:8080".parse().unwrap();
        let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let mut node = Node {
            address: address,
            peers: vec![old_peer_address.clone()],
        };

        let result = call(&mut node, new_peer_server.address());

        assert_eq!(Ok(()), result);
        mock.assert();

        let expected_result: Vec<SocketAddr> = vec![new_peer_server.address().clone()];
        assert_eq!(expected_result, node.peers);
    }
}
