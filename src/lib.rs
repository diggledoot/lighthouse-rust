mod post;

use axum::Router;
use hyper::{StatusCode, Uri};
use orm::*;
use std::error::Error;

pub fn create_base_router() -> Router {
    Router::new().fallback(fallback)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}

pub async fn database_migration(db_url: &str) -> Result<(), Box<dyn Error>> {
    make_migration(db_url).await?;
    Ok(())
}

pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    println!("Gracefully shutting down~");
}
