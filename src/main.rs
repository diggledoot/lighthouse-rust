use std::error::Error;

use axum::Router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().route("/", axum::routing::get(|| async { "Hello World" }));
    let server =
        axum::Server::bind(&"127.0.0.1:3000".parse().unwrap()).serve(app.into_make_service());
    server
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    println!("Gracefully shutting down~");
}
