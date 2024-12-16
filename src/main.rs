// Any module that is imported into here (e.g., `use abc_module;`) has its ancestry as the binary
// crate. Therefore, any function that is declared in the module (e.g., `abc_module`) under `pub(crate)`
// visibility can be accessed by the binary crate and NOT the library crate. If the module needs to be accessed
// by both crates, it needs to be made `pub`. The binary crate depends on the library crate (which has the same
// name listed in Cargo.toml); because stuff from library crate are imported in line 1 and 2.

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use log::{error, info};
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub mod services;

// Notes
// 1. Implement graceful shutdowns : https://hyper.rs/guides/1/server/graceful-shutdown/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Enable logging with env_logger wrapped around with Rust's log crate.
    // Set the logging level to INFO by default
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Read environment variable key PORT and set the value.
    // If no PORT environment varibale is set, the default is set, which is 3000.
    let port: u16 = env::var("PORT")
                        .map_or_else(|e| 3000, |v| v.parse().unwrap());

    // Bind the server to an address
    let address = SocketAddr::from(([127, 0, 00, 1], port));

    // Create a TcpListener and bind the address to it.
    let listener = TcpListener::bind(address).await?;

    info!("Server running on port: {:?}", port);

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
                error!("Error serving the connection: {:?}", err);
            }
        });
    }
}
