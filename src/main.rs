use std::error::Error;

use axum::Router;
use lighthouse::{fallback, post, shutdown_signal};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let db_url = dotenvy::var("DATABASE_URL")?;
    let ip_address_port = dotenvy::var("IP_ADDRESS_PORT")?;

    orm::make_migration_refresh(&db_url).await?;

    let app_routes = Router::new().nest("/post", post::post_router());
    let app = Router::new().nest("/api", app_routes).fallback(fallback);

    let server =
        axum::Server::bind(&ip_address_port.parse().unwrap()).serve(app.into_make_service());

    server
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    Ok(())
}
