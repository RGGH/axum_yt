use std::time::Duration;

// Import necessary libraries and modules
use sqlx::{postgres::PgPool, Pool, Error};
use crate::PgPoolOptions;



// Function to initialize and return a database connection pool
pub async fn init_db_pool() -> Result<PgPool, Error> {
    // Retrieve database connection string from environment variable
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:mysecretpassword@localhost:5432/mydatabase".to_string());

    let pool_result = PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(5))
    .connect(&db_connection_str)
    .await;

    let pool = match pool_result {
        Ok(pool) => pool,
        Err(err) => return Err(err.into())
        };
    Ok(pool)
}

pub async fn db_operation(pool: &PgPool) -> Result<Vec<String>, Error> {
    // Placeholder for your actual database operation logic
    // For example, let's pretend we're querying a table named "example_table"

    let query_result = sqlx::query!("SELECT * FROM notes")
        .fetch_all(pool)
        .await?;

    // Process the result and extract data
    let data: Vec<String> = query_result
        .iter()
        .map(|row| row.content.to_string())
        .collect();

    Ok(data)
}