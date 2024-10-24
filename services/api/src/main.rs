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

    let subscriber = tracing_subscriber::fmt::layer()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false);

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,rest_service=debug".into()),
        )
        .with(subscriber)
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
