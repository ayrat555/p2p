use crate::actions::add_peer::call as add_peer;
use crate::actions::broadcast_peer::call as broadcast_peer;
use crate::actions::return_peers::call as return_peers;
use crate::actions::whisper::call as whisper;
use crate::node;
use hyper::body;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::net::SocketAddr;

pub async fn start_server() -> Result<(), hyper::Error> {
    let node = node().lock().unwrap();

    let service = make_service_fn(move |conn: &AddrStream| {
        let addr = conn.remote_addr();
        async move {
            let addr = addr.clone();
            Ok::<_, hyper::Error>(service_fn(move |req| process(req, addr.clone())))
        }
    });

    let server = Server::bind(&node.address).serve(service);

    server.await?;

    Ok(())
}

async fn process(req: Request<Body>, address: SocketAddr) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/ping") => Ok(Response::new(Body::from("pong"))),

        (&Method::POST, "/connect") => {
            let bytes = body::to_bytes(req.into_body()).await?;
            let address: SocketAddr = String::from_utf8(bytes.to_vec()).unwrap().parse().unwrap();

            let mut node = node().lock().unwrap();

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

            let mut node = node().lock().unwrap();

            match add_peer(&mut node, &potential_peer) {
                Ok(_) => ok_status(),
                Err(err) => {
                    log::error!("Failed to add peer {:?}", err);
                    error_status()
                }
            }
        }

        (&Method::GET, "/fetch_peers") => {
            let node = node().lock().unwrap();
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
