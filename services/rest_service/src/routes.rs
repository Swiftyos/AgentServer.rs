use crate::handlers::project::{create_project, get_projects};
use axum::{
    routing::{get, post},
    Router,
};
use db::repository::PgProjectRepository;
use tower_http::compression::CompressionLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(
    crate::routes::root,
    crate::handlers::project::create_project,
    crate::handlers::project::get_projects
))]
#[openapi(components(schemas(
    crate::models::project::CreateProjectPayload,
    crate::models::project::GetProjectsParams
)))]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Hello world!", body = String)
    )
)]
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub fn create_routes() -> Router<PgProjectRepository> {
    Router::new()
        .route("/", get(root))
        .route("/projects", post(create_project::<PgProjectRepository>))
        .route("/projects", get(get_projects::<PgProjectRepository>))
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .layer(CompressionLayer::new())
}
