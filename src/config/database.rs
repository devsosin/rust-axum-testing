use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn create_connection_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Unable to connect to database");

    tracing::debug!("database connected successfully");
    
    pool
}