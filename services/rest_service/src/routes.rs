use crate::handlers::project::{create_project, get_projects};
use axum::{
    extract::{MatchedPath, Request},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use db::repository::PgProjectRepository;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use std::{future::ready, time::Instant};
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

fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}

async fn track_metrics(req: Request, next: Next) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::counter!("http_requests_total", &labels).increment(1);
    metrics::histogram!("http_requests_duration_seconds", &labels).record(latency);

    response
}

pub fn create_routes() -> Router<PgProjectRepository> {
    let recorder_handle = setup_metrics_recorder();

    Router::new()
        .route("/", get(root))
        .route("/projects", post(create_project::<PgProjectRepository>))
        .route("/projects", get(get_projects::<PgProjectRepository>))
        .route("/metrics", get(move || ready(recorder_handle.render())))
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .layer(middleware::from_fn(track_metrics))
        .layer(CompressionLayer::new())
}
