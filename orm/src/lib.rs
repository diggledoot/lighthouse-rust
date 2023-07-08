use std::error::Error;

use migrator::Migrator;
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;

pub mod entities;
pub mod migrator;
pub use sea_orm;

pub async fn make_migration_refresh(db_url: &str) -> Result<(), Box<dyn Error>> {
    let db = Database::connect(db_url).await?;
    Migrator::refresh(&db).await?;
    println!("Migration refresh ran~");
    Ok(())
}

pub async fn make_migration(db_url: &str) -> Result<(), Box<dyn Error>> {
    let db = Database::connect(db_url).await?;
    Migrator::up(&db, None).await?;
    println!("Migration up ran~");
    Ok(())
}
