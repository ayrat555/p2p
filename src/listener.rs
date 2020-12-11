use crate::node::Node;
use futures_util::TryStreamExt;
use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub struct Listener {
    node: Arc<Mutex<Node>>,
}

impl Listener {
    pub fn new(node: Node) -> Self {
        Listener {
            node: Arc::new(Mutex::new(node)),
        }
    }

    pub async fn start_server(&'static self) -> Result<(), hyper::Error> {
        let addr = self.node.lock().unwrap().address;
        let service = make_service_fn(|_| async move {
            Ok::<_, hyper::Error>(service_fn(move |req| process(req, self.node.clone())))
        });

        let server = Server::bind(&addr).serve(service);

        server.await
    }
}

async fn process(
    req: Request<Body>,
    node: Arc<Mutex<Node>>,
) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/ping") => Ok(Response::new(Body::from("pong"))),

        (&Method::POST, "/connect") => Ok(Response::new(Body::from("pong"))),

        (&Method::POST, "/add_peer") => {
            let bytes = body::to_bytes(req.into_body()).await?;
            let potential_peer: SocketAddr =
                String::from_utf8(bytes.to_vec()).unwrap().parse().unwrap();

            let mut unlocked_node = node.lock().unwrap();

            if !unlocked_node.peer_exists(&potential_peer) {
                unlocked_node.add_peer(potential_peer);
            }

            println!("{:?}", unlocked_node);
            //
            log::error!("{:?}", node);

            Ok(Response::new("ok".into()))
        }

        (&Method::GET, "/get_peers") => Ok(Response::new(Body::from("pong"))),

        (&Method::GET, "/whisper") => Ok(Response::new(Body::from("pong"))),

        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),

        // Convert to uppercase before sending back to client using a stream.
        (&Method::POST, "/echo/uppercase") => {
            let chunk_stream = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
            Ok(Response::new(Body::wrap_stream(chunk_stream)))
        }

        // Reverse the entire body before sending back to the client.
        //
        // Since we don't know the end yet, we can't simply stream
        // the chunks as they arrive as we did with the above uppercase endpoint.
        // So here we do `.await` on the future, waiting on concatenating the full body,
        // then afterwards the content can be reversed. Only then can we return a `Response`.
        (&Method::POST, "/echo/reversed") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;

            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
            Ok(Response::new(Body::from(reversed_body)))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
