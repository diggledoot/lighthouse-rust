use std::error::Error;

use lighthouse::{create_base_router, database_migration, shutdown_signal};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let db_url = dotenvy::var("DATABASE_URL")?;
    let ip_address_port = dotenvy::var("IP_ADDRESS_PORT")?;
    database_migration(&db_url).await?;
    let app = create_base_router();
    let server =
        axum::Server::bind(&ip_address_port.parse().unwrap()).serve(app.into_make_service());
    server
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    Ok(())
}
