// Any module that is imported into here (e.g., `use abc_module;`) has its ancestry as the binary
// crate. Therefore, any function that is declared in the module (e.g., `abc_module`) under `pub(crate)`
// visibility can be accessed by the binary crate and NOT the library crate. If the module needs to be accessed
// by both crates, it needs to be made `pub`. The binary crate depends on the library crate (which has the same
// name listed in Cargo.toml); because stuff from library crate are imported in line 1 and 2.

use std::convert::Infallible;
use hyper::body::Bytes;
use std::net::SocketAddr;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper::service::service_fn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // Bind the server to an address
    let address = SocketAddr::from(([127, 0, 00, 1], 3000));

    // Create a TcpListener and bind the address to it.
    let listener = TcpListener::bind(address).await?;

    println!("Server running");

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        // Spaw na tokio task for concurrent processing of incoming streams
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(io, service_fn(hello)).await {
                eprintln!("Error serving the connection: {:?}", err);
            }
        });
    }
}

async fn hello(_: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}