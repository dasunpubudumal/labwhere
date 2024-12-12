// Any module that is imported into here (e.g., `use abc_module;`) has its ancestry as the binary
// crate. Therefore, any function that is declared in the module (e.g., `abc_module`) under `pub(crate)`
// visibility can be accessed by the binary crate and NOT the library crate. If the module needs to be accessed
// by both crates, it needs to be made `pub`. The binary crate depends on the library crate (which has the same
// name listed in Cargo.toml); because stuff from library crate are imported in line 1 and 2.

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub mod services;

// Notes
// 1. Implement graceful shutdowns : https://hyper.rs/guides/1/server/graceful-shutdown/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Bind the server to an address
    let address = SocketAddr::from(([127, 0, 00, 1], 3000));

    // Create a TcpListener and bind the address to it.
    let listener = TcpListener::bind(address).await?;

    println!("Server running..");

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        // Spawn tokio task for concurrent processing of incoming streams
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                // This is the global service handler.
                // This service handler should delegate the request to the relevant endpoint
                .serve_connection(io, service_fn(services::scan::scan))
                .await
            {
                eprintln!("Error serving the connection: {:?}", err);
            }
        });
    }
}
