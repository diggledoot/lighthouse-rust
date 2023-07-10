pub mod post;
pub mod utilities;

use axum::{routing::{IntoMakeService, get}, Router};
use hyper::{StatusCode, Uri, Server, server::conn::AddrIncoming};

pub async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}

pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    println!("Gracefully shutting down~");
}

pub fn run()->Server<AddrIncoming,IntoMakeService<Router>>{
    todo!("Move the run logic here for slim main")
}
