use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Establishes a connection to the PostgreSQL database.
///
/// This function creates a connection pool to the specified database URL.
/// It sets a maximum of 5 connections and a 3-second timeout for acquiring a connection.
///
/// # Arguments
///
/// * `database_url` - A string slice that holds the URL of the database to connect to.
///
/// # Returns
///
/// * `Result<PgPool>` - A Result containing the connection pool if successful, or an error if the connection fails.
///
/// # Errors
///
/// This function will return an error if the connection to the database cannot be established.
pub async fn establish_connection(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::{Config, Environment, File};
    use tokio;

    #[tokio::test]
    async fn test_establish_connection() {
        let config = Config::builder()
            .add_source(File::with_name("../../config/test.toml"))
            .add_source(Environment::with_prefix("APP"))
            .build()
            .expect("Failed to load configuration");

        let database_url = config
            .get_string("database_url")
            .expect("DATABASE_URL must be set in config");

        let result = establish_connection(&database_url).await;
        assert!(result.is_ok());
    }
}
