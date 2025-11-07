pub mod models;
pub mod repositories;

use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;

pub use models::*;
pub use repositories::*;

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    
    Ok(())
}
