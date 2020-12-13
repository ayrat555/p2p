use crate::actions::add_peer::call as add_peer;
use crate::actions::broadcast_peer::call as broadcast_peer;
use crate::actions::return_peers::call as return_peers;
use crate::actions::whisper::call as whisper;
use crate::node;
use crate::node::Node;
use hyper::body;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub async fn start_server(node: Arc<Mutex<Node>>, address: SocketAddr) -> Result<(), hyper::Error> {
    let service = make_service_fn(move |conn: &AddrStream| {
        let addr = conn.remote_addr();
        let node = node.clone();
        async move {
            let addr = addr.clone();
            Ok::<_, hyper::Error>(service_fn(move |req| process(req, addr, node.clone())))
        }
    });

    let server = Server::bind(&address).serve(service);
    server.await?;

    Ok(())
}

async fn process(
    req: Request<Body>,
    address: SocketAddr,
    node: Arc<Mutex<Node>>,
) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/ping") => Ok(Response::new(Body::from("pong"))),

        (&Method::POST, "/connect") => {
            let bytes = body::to_bytes(req.into_body()).await?;
            let address: SocketAddr = String::from_utf8(bytes.to_vec()).unwrap().parse().unwrap();

            let mut node = node.lock().unwrap();

            match broadcast_peer(&mut node, &address) {
                Ok(_) => ok_status(),
                Err(err) => {
                    log::error!("Failed to broadcast address {:?}", err);
                    error_status()
                }
            }
        }

        (&Method::POST, "/add_peer") => {
            let bytes = body::to_bytes(req.into_body()).await?;
            let potential_peer: SocketAddr =
                String::from_utf8(bytes.to_vec()).unwrap().parse().unwrap();

            let mut node = node.lock().unwrap();

            match add_peer(&mut node, &potential_peer) {
                Ok(_) => ok_status(),
                Err(err) => {
                    log::error!("Failed to add peer {:?}", err);
                    error_status()
                }
            }
        }

        (&Method::GET, "/fetch_peers") => {
            let node = node.lock().unwrap();
            let peers_response = return_peers(&node);

            Ok(Response::new(Body::from(peers_response)))
        }

        (&Method::GET, "/whisper") => {
            let bytes = body::to_bytes(req.into_body()).await?;
            let message: String = String::from_utf8(bytes.to_vec()).unwrap();

            whisper(&message, &address);

            ok_status()
        }

        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn ok_status() -> Result<Response<Body>, hyper::Error> {
    let mut ok = Response::default();
    *ok.status_mut() = StatusCode::OK;
    Ok(ok)
}

fn error_status() -> Result<Response<Body>, hyper::Error> {
    let mut error = Response::default();
    *error.status_mut() = StatusCode::BAD_REQUEST;
    Ok(error)
}

#[cfg(test)]
mod tests {
    use super::node::Node;
    use super::start_server;
    use crate::actions::client;
    use httpmock::Method::GET;
    use httpmock::Method::POST;
    use httpmock::MockServer;
    use isahc::ResponseExt;
    use std::net::SocketAddr;
    use std::sync::{Arc, Mutex};
    use tokio::runtime;

    #[test]
    fn ping_returns_pong() {
        let node_address: SocketAddr = "127.0.0.1:8083".parse().unwrap();
        let node = Arc::new(Mutex::new(Node::new(node_address.clone())));

        let tokio_runtime = runtime::Builder::new()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();

        tokio_runtime.spawn(start_server(node, node_address.clone()));

        let action_path = format!("http://{}/ping", node_address);
        let mut response = client().get(action_path).unwrap();

        let body = response.text().unwrap();

        assert_eq!(response.status(), 200);
        assert_eq!("pong", body);
    }

    #[test]
    fn connect_broadcasts_and_adds_peer() {
        let peer_server = MockServer::start();
        let mock = peer_server.mock(|when, then| {
            when.method(GET).path("/ping");
            then.status(200)
                .header("Content-Type", "text/html")
                .body("pong");
        });

        let peer_server1 = MockServer::start();
        let mock1 = peer_server1.mock(|when, then| {
            when.method(POST).path("/add_peer");
            then.status(200);
        });

        let node_address: SocketAddr = "127.0.0.1:8084".parse().unwrap();
        let node = Arc::new(Mutex::new(Node {
            address: node_address.clone(),
            peers: vec![peer_server1.address().clone()],
        }));

        let tokio_runtime = runtime::Builder::new()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();

        tokio_runtime.spawn(start_server(node.clone(), node_address.clone()));

        let action_path = format!("http://{}/connect", node_address);
        let response = client()
            .post(action_path, peer_server.address().to_string())
            .unwrap();

        assert_eq!(response.status(), 200);
        assert_eq!(
            node.lock().unwrap().peers,
            vec!(
                peer_server1.address().clone(),
                peer_server.address().clone()
            )
        );
        mock.assert();
        mock1.assert();
    }

    #[test]
    fn add_peer_adds_peer() {
        let peer_server = MockServer::start();
        let mock = peer_server.mock(|when, then| {
            when.method(GET).path("/ping");
            then.status(200)
                .header("Content-Type", "text/html")
                .body("pong");
        });

        let node_address: SocketAddr = "127.0.0.1:8085".parse().unwrap();
        let node = Arc::new(Mutex::new(Node {
            address: node_address.clone(),
            peers: vec![],
        }));

        let tokio_runtime = runtime::Builder::new()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();

        tokio_runtime.spawn(start_server(node.clone(), node_address.clone()));

        let action_path = format!("http://{}/add_peer", node_address);
        let response = client()
            .post(action_path, peer_server.address().to_string())
            .unwrap();

        assert_eq!(response.status(), 200);
        assert_eq!(
            node.lock().unwrap().peers,
            vec!(peer_server.address().clone())
        );
        mock.assert();
    }

    #[test]
    fn fetch_peers_returns_peers() {
        let node_address: SocketAddr = "127.0.0.1:8090".parse().unwrap();
        let peers: Vec<SocketAddr> = vec![
            "127.0.0.1:8086".parse().unwrap(),
            "127.0.0.1:8087".parse().unwrap(),
        ];
        let node = Arc::new(Mutex::new(Node {
            address: node_address.clone(),
            peers: peers.clone(),
        }));

        let tokio_runtime = runtime::Builder::new()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();

        tokio_runtime.spawn(start_server(node.clone(), node_address.clone()));

        let action_path = format!("http://{}/fetch_peers", node_address);
        let mut response = client().get(action_path).unwrap();

        let body = response.text().unwrap();

        assert_eq!(response.status(), 200);
        assert_eq!("127.0.0.1:8086,127.0.0.1:8087", body);
    }

    #[test]
    fn whisper_returns_ok() {
        let node_address: SocketAddr = "127.0.0.1:8089".parse().unwrap();
        let node = Arc::new(Mutex::new(Node::new(node_address.clone())));

        let tokio_runtime = runtime::Builder::new()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();

        tokio_runtime.spawn(start_server(node.clone(), node_address.clone()));

        let action_path = format!("http://{}/whisper", node_address);
        let response = client().get(action_path).unwrap();

        assert_eq!(response.status(), 200);
    }
}
