use crate::actions;
use crate::actions::Error;
use crate::node::Node;
use std::net::SocketAddr;

pub fn call(node: &mut Node) -> Result<(), Error> {
    let client = actions::client();
    let mut nodes_to_remove: Vec<SocketAddr> = vec![];
    let message = format!("hello from {}", node.address.to_string());

    for peer in &node.peers {
        let action_path = format!("http://{}/whisper", peer.to_string());

        match client.post(action_path, message.clone()) {
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

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::call;
    use crate::node::Node;
    use httpmock::Method::POST;
    use httpmock::MockServer;
    use std::net::SocketAddr;

    #[test]
    fn call_returns_success() {
        let peer_server = MockServer::start();
        let mock = peer_server.mock(|when, then| {
            when.method(POST).path("/whisper");
            then.status(200);
        });

        let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let mut node = Node {
            address: address,
            peers: vec![peer_server.address().clone()],
        };

        let result = call(&mut node);

        assert_eq!(Ok(()), result);
        mock.assert();

        let expected_result: Vec<SocketAddr> = vec![peer_server.address().clone()];
        assert_eq!(expected_result, node.peers);
    }

    #[test]
    fn call_removes_peer_if_fails_to_respond() {
        let old_peer_address: SocketAddr = "127.0.0.2:8080".parse().unwrap();
        let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let mut node = Node {
            address: address,
            peers: vec![old_peer_address.clone()],
        };

        let result = call(&mut node);

        assert_eq!(Ok(()), result);

        let expected_result: Vec<SocketAddr> = vec![];
        assert_eq!(expected_result, node.peers);
    }
}
