use anyhow::Result;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use std::time::Duration;
use tracing::info;

/// Creates a connection pool to a PostgreSQL database.
///
/// This function establishes a connection pool to a PostgreSQL database using the provided
/// database URL and optional schema name. It configures the pool with a maximum of 5 connections
/// and a 3-second acquisition timeout.
///
/// # Arguments
///
/// * `database_url` - A string slice that holds the URL of the PostgreSQL database.
/// * `schema` - An optional string slice specifying the schema to use. If provided, it will be
///              set as the search path for the database connection.
///
/// # Returns
///
/// Returns a `Result` containing a `PgPool` if the connection is successful, or an error if it fails.
///
/// # Examples
///
/// ```
/// use db::connection;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let database_url = "postgres://username:password@localhost/database"; // pragma: allowlist secret
///     let schema = Some("public");
///     
///     let pool = connection::create_pool(database_url, schema).await?;
///     // Use the pool for database operations
///     Ok(())
/// }
/// ```

pub async fn create_pool(database_url: &str, schema: Option<&str>) -> Result<PgPool> {
    let mut options: PgConnectOptions = database_url.parse()?;

    if let Some(schema_name) = schema {
        // Create the schema if it doesn't exist
        let temp_pool = PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_secs(3))
            .connect_with(options.clone())
            .await?;

        sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name))
            .execute(&temp_pool)
            .await?;

        options = options.options([("search_path", schema_name)]);
        info!("Schema created and set: {}", schema_name);
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(options)
        .await?;

    Ok(pool)
}

/// Applies all pending SQL migrations to the database.
///
/// This function runs all the SQL migrations found in the "migrations" directory
/// using SQLx. It ensures that the database schema is up-to-date with the latest
/// changes defined in the migration files.
///
/// # Arguments
///
/// * `pool` - A reference to a `PgPool` representing the database connection pool.
///
/// # Returns
///
/// Returns a `Result<()>` which is `Ok(())` if all migrations were successfully
/// applied, or an `Error` if there was a problem applying the migrations.
///
/// # Examples
///
/// ```
/// use db::connection;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let database_url = "postgres://username:password@localhost/database"; // pragma: allowlist secret
///     let pool = connection::create_pool(database_url, None).await?;
///     
///     connection::apply_migrations(&pool).await?;
///     println!("Migrations applied successfully");
///     Ok(())
/// }
/// ```
pub async fn apply_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to apply migrations: {}", e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::{Config, Environment, File};
    use tokio;

    #[tokio::test]
    async fn test_establish_connection_with_schema() {
        let config = Config::builder()
            .add_source(File::with_name("../../config/test.toml"))
            .add_source(Environment::with_prefix("APP"))
            .build()
            .expect("Failed to load configuration");

        let database_url = config
            .get_string("database_url")
            .expect("DATABASE_URL must be set in config");

        let schema = Some("your_schema"); // Replace with your schema name or load from config

        let result = create_pool(&database_url, schema).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_establish_connection_without_schema() {
        let config = Config::builder()
            .add_source(File::with_name("../../config/test.toml"))
            .add_source(Environment::with_prefix("APP"))
            .build()
            .expect("Failed to load configuration");

        let database_url = config
            .get_string("database_url")
            .expect("DATABASE_URL must be set in config");

        let result = create_pool(&database_url, None).await;
        assert!(result.is_ok());
    }
}
