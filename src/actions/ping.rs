use crate::actions;
use crate::actions::Error;
use isahc::ResponseExt;
use std::net::SocketAddr;

pub fn call(address: &SocketAddr) -> Result<(), Error> {
    let client = actions::client();
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
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use std::net::SocketAddr;

    #[test]
    fn call_returns_success() {
        let server = MockServer::start();

        server.mock(|when, then| {
            when.method(GET).path("/ping");
            then.status(200)
                .header("Content-Type", "text/html")
                .body("pong");
        });

        let result = call(server.address());

        assert_eq!(Ok(()), result);
    }

    #[test]
    fn call_returns_failure_if_address_is_not_available() {
        let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let result = call(&address);

        assert_eq!(
            Err(Error {
                msg: "ConnectFailed: failed to connect to the server".to_string()
            }),
            result
        );
    }
}
