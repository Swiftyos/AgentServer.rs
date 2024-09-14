use axum::{
    routing::{get, post},
    Router,
};
use db::{connection, models::project, queries::project_queries};
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use utoipa::{IntoParams, OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(root, create_project, get_projects))]
#[openapi(components(schemas(CreateProjectPayload, GetProjectsParams)))]
struct ApiDoc;

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

    // Build our application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/projects", post(create_project))
        .route("/projects", get(get_projects))
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .layer(CompressionLayer::new())
        .with_state(pool);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[utoipa::path(
    get,
    path = "/hello",
    responses(
        (status = 200, description = "Hello world!", body = String)
    )
)]
async fn root() -> &'static str {
    "Hello, World!"
}

#[utoipa::path(
    post,
    path = "/projects",
    request_body = CreateProjectPayload,
    responses(
        (status = 200, description = "Project created successfully", body = Project)
    )
)]
async fn create_project(
    axum::extract::State(pool): axum::extract::State<PgPool>,
    axum::Json(payload): axum::Json<CreateProjectPayload>,
) -> Result<axum::Json<project::Project>, axum::http::StatusCode> {
    let project =
        project_queries::create_project(&pool, &payload.name, payload.description.as_deref())
            .await
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(project))
}

#[utoipa::path(
    get,
    path = "/projects",
    params(GetProjectsParams),
    responses(
        (status = 200, description = "Projects fetched successfully", body = Vec<Project>)
    )
)]
async fn get_projects(
    axum::extract::State(pool): axum::extract::State<PgPool>,
    axum::extract::Query(params): axum::extract::Query<GetProjectsParams>,
) -> Result<axum::Json<Vec<project::Project>>, axum::http::StatusCode> {
    let projects = project_queries::get_projects(&pool, params.page, params.page_size)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::Json(projects))
}

// Structs for request payloads
#[derive(serde::Deserialize, IntoParams, ToSchema)]
struct CreateProjectPayload {
    name: String,
    description: Option<String>,
}

#[derive(serde::Deserialize, IntoParams, ToSchema)]
struct GetProjectsParams {
    page: i64,
    page_size: i64,
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::{
//         body::Body,
//         http::{Request, StatusCode},
//         routing::Router,
//     };
//     use serde_json::json;
//     use uuid::Uuid;
//     // Mock database pool
//     struct MockPool;

//     impl MockPool {
//         fn new() -> Self {
//             MockPool
//         }
//     }

//     impl std::fmt::Debug for MockPool {
//         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//             f.debug_struct("MockPool").finish()
//         }
//     }

//     // Mock project queries
//     #[derive(Clone)]
//     struct MockProjectQueries;

//     impl MockProjectQueries {
//         async fn create_project(
//             _pool: &MockPool,
//             name: &str,
//             description: Option<&str>,
//         ) -> Result<project::Project, anyhow::Error> {
//             Ok(project::Project {
//                 id: Uuid::new_v4(),
//                 name: name.to_string(),
//                 description: description.map(String::from),
//                 created_at: chrono::Utc::now(),
//                 updated_at: chrono::Utc::now(),
//             })
//         }

//         async fn get_projects(
//             _pool: &MockPool,
//             _page: i64,
//             _page_size: i64,
//         ) -> Result<Vec<project::Project>, anyhow::Error> {
//             Ok(vec![
//                 project::Project {
//                     id: Uuid::new_v4(),
//                     name: "Test Project".to_string(),
//                     description: Some("Test Description".to_string()),
//                     created_at: chrono::Utc::now(),
//                     updated_at: chrono::Utc::now(),
//                 },
//                 project::Project {
//                     id: Uuid::new_v4(),
//                     name: "Another Project".to_string(),
//                     description: None,
//                     created_at: chrono::Utc::now(),
//                     updated_at: chrono::Utc::now(),
//                 },
//             ])
//         }
//     }

//     // Helper function to create a test app
//     fn app() -> Router {
//         Router::new()
//             .route("/projects", axum::routing::post(create_project))
//             .route("/projects", axum::routing::get(get_projects))
//             .with_state(MockPool::new())
//     }

//     #[tokio::test]
//     async fn test_create_project() {
//         let app = app();

//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .method("POST")
//                     .uri("/projects")
//                     .header("Content-Type", "application/json")
//                     .body(Body::from(
//                         json!({
//                             "name": "New Project",
//                             "description": "Project Description"
//                         })
//                         .to_string(),
//                     ))
//                     .unwrap(),
//             )
//             .await
//             .unwrap();

//         assert_eq!(response.status(), StatusCode::OK);

//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//         let project: project_queries::Project = serde_json::from_slice(&body).unwrap();

//         assert_eq!(project.name, "New Project");
//         assert_eq!(project.description, Some("Project Description".to_string()));
//     }

//     #[tokio::test]
//     async fn test_get_projects() {
//         let app = app();

//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .method("GET")
//                     .uri("/projects?page=1&page_size=10")
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();

//         assert_eq!(response.status(), StatusCode::OK);

//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//         let projects: Vec<project_queries::Project> = serde_json::from_slice(&body).unwrap();

//         assert_eq!(projects.len(), 2);
//         assert_eq!(projects[0].name, "Test Project");
//         assert_eq!(projects[1].name, "Another Project");
//     }
//}
