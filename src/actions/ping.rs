use crate::actions;
use crate::actions::Error;
use crate::node::Node;
use isahc::ResponseExt;
use std::net::SocketAddr;

pub fn call(node: &Node, address: &SocketAddr) -> Result<(), Error> {
    let client = actions::client(&node.address);
    let action_path = format!("http://{}/ping", address.to_string());

    match client.get(action_path) {
        Ok(mut response) => {
            if response.status() == 200 && response.text().unwrap() == "pong" {
                Ok(())
            } else {
                let msg = "Failed to receive pong".to_string();
                Err(Error { msg })
            }
        }

        Err(err) => {
            let msg = format!("{}", err);
            Err(Error { msg })
        }
    }
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
        let node = Node::new(address);

        let result = call(&node, server.address());

        assert_eq!(Ok(()), result);
        mock.assert();
    }

    #[test]
    fn call_returns_failure_if_address_is_not_available() {
        let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let node_address: SocketAddr = "127.0.0.1:8081".parse().unwrap();
        let node = Node::new(node_address);
        let result = call(&node, &address);

        assert_eq!(
            Err(Error {
                msg: "ConnectFailed: failed to connect to the server".to_string()
            }),
            result
        );
    }
}
