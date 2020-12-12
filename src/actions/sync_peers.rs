use crate::actions;
use crate::actions::add_peer::call as add_peer;
use crate::actions::Error;
use crate::node::Node;
use isahc::ResponseExt;
use std::net::SocketAddr;

pub fn call(node: &mut Node) -> Result<(), Error> {
    let client = actions::client();
    let mut nodes_to_remove: Vec<SocketAddr> = vec![];
    let mut nodes_to_add: Vec<SocketAddr> = vec![];

    for peer in &node.peers {
        let action_path = format!("http://{}/fetch_peers", peer.to_string());

        match client.get(action_path) {
            Ok(mut response) => match u16::from(response.status()) {
                200 => {
                    let text = response.text().unwrap();
                    let mut found_peers: Vec<SocketAddr> = parse_peers(&text);

                    nodes_to_add.append(&mut found_peers);
                }
                _ => nodes_to_remove.push(peer.clone()),
            },

            Err(_err) => nodes_to_remove.push(peer.clone()),
        }
    }

    for peer in &nodes_to_remove {
        node.remove_peer(peer);
    }

    for peer in &nodes_to_add {
        if let Err(error) = add_peer(node, peer) {
            log::debug!(
                "Failed to add peer {} because of {:?}",
                peer.to_string(),
                error
            );
        }
    }

    Ok(())
}

fn parse_peers(response: &str) -> Vec<SocketAddr> {
    response
        .split(",")
        .map(|peer| {
            let peer_addr: Result<SocketAddr, std::net::AddrParseError> = peer.parse();
            peer_addr
        })
        .filter_map(Result::ok)
        .collect::<Vec<SocketAddr>>()
}

#[cfg(test)]
mod tests {
    use super::call;
    use crate::node::Node;
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use std::net::SocketAddr;

    #[test]
    fn call_adds_two_peers() {
        let new_peer1_server = MockServer::start();
        let mock1 = new_peer1_server.mock(|when, then| {
            when.method(GET).path("/ping");
            then.status(200)
                .header("Content-Type", "text/html")
                .body("pong");
        });

        let new_peer2_server = MockServer::start();
        let mock2 = new_peer2_server.mock(|when, then| {
            when.method(GET).path("/ping");
            then.status(200)
                .header("Content-Type", "text/html")
                .body("pong");
        });

        let old_peer_server = MockServer::start();
        let mock3 = old_peer_server.mock(|when, then| {
            when.method(GET).path("/fetch_peers");
            then.status(200).body(format!(
                "{},{}",
                new_peer1_server.address(),
                new_peer2_server.address()
            ));
        });

        let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let mut node = Node {
            address: address,
            peers: vec![old_peer_server.address().clone()],
        };

        let result = call(&mut node);

        assert_eq!(Ok(()), result);
        mock1.assert();
        mock2.assert();
        mock3.assert();

        let expected_result: Vec<SocketAddr> = vec![
            old_peer_server.address().clone(),
            new_peer1_server.address().clone(),
            new_peer2_server.address().clone(),
        ];
        assert_eq!(expected_result, node.peers);
    }

    #[test]
    fn call_removes_peers_that_fails_to_return_peers() {
        let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let peer_address: SocketAddr = "127.0.0.1:8081".parse().unwrap();
        let mut node = Node {
            address: address,
            peers: vec![peer_address.clone()],
        };

        let result = call(&mut node);

        assert_eq!(Ok(()), result);

        let expected_result: Vec<SocketAddr> = vec![];
        assert_eq!(expected_result, node.peers);
    }
}
