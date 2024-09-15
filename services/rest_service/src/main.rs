use db::connection;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod handlers;
mod models;
mod routes;
mod srv_config;

use db::repository::PgProjectRepository;
use routes::create_routes;

#[tokio::main]
async fn main() {
    // Load config
    let config = match srv_config::RestConfig::new() {
        Ok(cfg) => cfg,
        Err(e) => {
            tracing::error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,rest_service=debug".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    // Set up database connection
    let pool = connection::create_pool(&config.database_url, Some(&config.database_schema))
        .await
        .expect("Failed to create database pool");

    // Apply migrations
    connection::apply_migrations(&pool)
        .await
        .expect("Failed to apply migrations");

    let repo = PgProjectRepository::new(pool);

    // Build our application with routes
    let app = create_routes().with_state(repo);

    // Run it
    let host: std::net::IpAddr = config.server_host.parse().expect("Invalid host address");
    let port = config.server_port;

    let addr = SocketAddr::from((host, port));

    tracing::info!("listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
