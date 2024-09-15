use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use db::{models::project, queries::project_queries};
use sqlx::PgPool;

use crate::models::project::{CreateProjectPayload, GetProjectsParams};

#[utoipa::path(
    post,
    path = "/projects",
    request_body = CreateProjectPayload,
    responses(
        (status = 200, description = "Project created successfully", body = Project)
    )
)]
pub async fn create_project(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProjectPayload>,
) -> Result<Json<project::Project>, StatusCode> {
    let project =
        project_queries::create_project(&pool, &payload.name, payload.description.as_deref())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(project))
}

#[utoipa::path(
    get,
    path = "/projects",
    params(GetProjectsParams),
    responses(
        (status = 200, description = "Projects fetched successfully", body = Vec<Project>)
    )
)]
pub async fn get_projects(
    State(pool): State<PgPool>,
    Query(params): Query<GetProjectsParams>,
) -> Result<Json<Vec<project::Project>>, StatusCode> {
    let projects = project_queries::get_projects(&pool, params.page, params.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(projects))
}
