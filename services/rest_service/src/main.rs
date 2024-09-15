use db::connection;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod handlers;
mod models;
mod routes;

use db::repository::PgProjectRepository;
use routes::create_routes;

#[tokio::main]
async fn main() {
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
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = connection::create_pool(&database_url, None)
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
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
