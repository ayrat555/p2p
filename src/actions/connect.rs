use crate::actions;
use crate::actions::add_peer::call as add_peer;
use crate::actions::sync_peers::call as sync_peers;
use crate::actions::Error;
use crate::node::Node;
use std::net::SocketAddr;

pub fn call(node: &mut Node, address: &SocketAddr) -> Result<(), Error> {
    let client = actions::client();

    let action_path = format!("http://{}/connect", address.to_string());

    match client.post(action_path, node.address.to_string()) {
        Ok(response) => {
            if response.status() != 200 {
                let msg = format!("Failed to connect to bootnode {:?}", response);

                log::error!("{}", msg);

                return Err(Error { msg });
            }

            add_peer(node, address)?;

            sync_peers(node)
        }

        Err(err) => {
            let msg = format!("Failed to connect to bootnode {:?}", err);

            log::error!("{}", msg);

            return Err(Error { msg });
        }
    }
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
    fn call_adds_peers_and_sync_peers() {
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

        let mock3 = new_peer1_server.mock(|when, then| {
            when.method(GET).path("/fetch_peers");
            then.status(200)
                .header("Content-Type", "text/html")
                .body(new_peer2_server.address().to_string());
        });

        let mock4 = new_peer1_server.mock(|when, then| {
            when.method(POST).path("/connect");
            then.status(200);
        });

        let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let mut node = Node {
            address: address,
            peers: vec![],
        };

        let result = call(&mut node, new_peer1_server.address());

        assert_eq!(Ok(()), result);
        mock1.assert();
        mock2.assert();
        mock3.assert();
        mock4.assert();

        let expected_result: Vec<SocketAddr> = vec![
            new_peer1_server.address().clone(),
            new_peer2_server.address().clone(),
        ];
        assert_eq!(expected_result, node.peers);
    }
}
