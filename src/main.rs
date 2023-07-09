mod app_state;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use lighthouse::app_state::AppState;
use lighthouse::{fallback, post, shutdown_signal};
use orm::sea_orm::Database;
use post::handlers::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let db_url = dotenvy::var("DATABASE_URL")?;
    let ip_address_port = dotenvy::var("IP_ADDRESS_PORT")?;

    orm::make_migration_refresh(&db_url).await?;

    let db_pool = Database::connect(&db_url).await?;
    let state = AppState { conn: db_pool };

    let post_routes = Router::new()
        .route("/", post(create_post))
        .route("/:id", get(get_post_by_id))
        .route("/:id", put(update_post_by_id))
        .route("/:id", delete(delete_post_by_id));

    let app_routes = Router::new().nest("/post", post_routes);

    let app = Router::new()
        .nest("/api", app_routes)
        .with_state(state)
        .fallback(fallback);

    let server =
        axum::Server::bind(&ip_address_port.parse().unwrap()).serve(app.into_make_service());

    server
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    Ok(())
}
