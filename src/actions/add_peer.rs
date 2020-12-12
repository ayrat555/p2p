use crate::actions::ping::call as ping;
use crate::actions::Error;
use crate::node::Node;
use std::net::SocketAddr;

pub fn call(node: &mut Node, address: &SocketAddr) -> Result<(), Error> {
    ping(&address)?;

    if !node.peer_exists(address) {
        node.add_peer(address.clone());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::call;
    use crate::actions::Error;
    use crate::node::Node;
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use std::net::SocketAddr;

    #[test]
    fn call_returns_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/ping");
            then.status(200)
                .header("Content-Type", "text/html")
                .body("pong");
        });
        let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let mut node = Node::new(address);

        let result = call(&mut node, server.address());

        assert_eq!(Ok(()), result);

        let expected_result: Vec<SocketAddr> = vec![server.address().clone()];
        assert_eq!(expected_result, node.peers);
        mock.assert();
    }

    #[test]
    fn call_fails_to_add_peer() {
        let address1: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let address2: SocketAddr = "127.0.0.1:8081".parse().unwrap();
        let mut node = Node::new(address1);

        let result = call(&mut node, &address2);

        assert_eq!(
            Err(Error {
                msg: "ConnectFailed: failed to connect to the server".to_string()
            }),
            result
        );

        let expected_result: Vec<SocketAddr> = vec![];
        assert_eq!(expected_result, node.peers);
    }
}
