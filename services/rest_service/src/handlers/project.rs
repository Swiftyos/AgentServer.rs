use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    response::Response,
    Json,
};
use db::models::project;
use db::repository::ProjectRepository;
use tracing::{info, instrument};

use crate::models::project::{CreateProjectPayload, GetProjectsParams};

#[utoipa::path(
    post,
    path = "/projects",
    request_body = CreateProjectPayload,
    responses(
        (status = 200, description = "Project created successfully", body = Project)
    )
)]
#[instrument(
    name = "create_project",
    skip(repo),
    fields(
        project_name = %payload.name,
        project_description = %payload.description
    )
)]
pub async fn create_project<R: ProjectRepository>(
    State(repo): State<R>,
    Json(payload): Json<CreateProjectPayload>,
) -> Result<Json<project::Project>, Response> {
    if payload.name.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Project name cannot be empty").into_response());
    }

    if payload.description.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Project description cannot be empty",
        )
            .into_response());
    }
    info!("Creating project with name: {}", payload.name);

    let project = repo
        .create_project(&payload.name, &payload.description)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();

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
#[instrument(
    name = "get_projects",
    skip(repo),
    fields(
        page = ?params.page,
        page_size = ?params.page_size
    )
)]
pub async fn get_projects<R: ProjectRepository>(
    State(repo): State<R>,
    Query(params): Query<GetProjectsParams>,
) -> Result<Json<Vec<project::Project>>, StatusCode> {
    info!(
        "Fetching projects with page: {:?}, page_size: {:?}",
        params.page, params.page_size
    );
    let projects = repo
        .get_projects(params.page, params.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(projects))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Error;
    use async_trait::async_trait;
    use axum::Json;
    use db::repository::ProjectRepository;
    use uuid::Uuid;

    #[derive(Clone)]
    struct MockProjectRepository;

    #[async_trait]
    impl ProjectRepository for MockProjectRepository {
        async fn create_project(
            &self,
            name: &str,
            description: &str,
        ) -> Result<project::Project, Error> {
            let new_project = project::Project {
                id: Uuid::new_v4(),
                name: name.to_string(),
                description: description.to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            Ok(new_project)
        }

        async fn get_projects(
            &self,
            _page: Option<i32>,
            page_size: Option<i32>,
        ) -> Result<Vec<project::Project>, Error> {
            let page_size = page_size.unwrap_or(10) as usize;

            let mut projects = Vec::new();
            for i in 0..page_size {
                projects.push(project::Project {
                    id: Uuid::new_v4(),
                    name: format!("Project {}", i + 1),
                    description: format!("Description for Project {}", i + 1),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                });
            }
            Ok(projects)
        }
    }

    #[tokio::test]
    async fn test_create_project() {
        let repo = MockProjectRepository;

        // Create a test payload
        let payload = CreateProjectPayload {
            name: "Test Project".to_string(),
            description: "A test project description".to_string(),
        };

        // Call the create_project handler
        let response = create_project(State(repo), Json(payload)).await;

        // Check the response
        assert!(response.is_ok());
        let created_project = response.unwrap();

        assert_eq!(created_project.name, "Test Project");
        assert_eq!(
            created_project.description,
            "A test project description".to_string()
        );
    }

    #[tokio::test]
    async fn test_create_project_with_empty_name() {
        let repo = MockProjectRepository;

        // Create a test payload with an empty name
        let payload = CreateProjectPayload {
            name: "".to_string(),
            description: "A test project description".to_string(),
        };

        // Call the create_project handler
        let response = create_project(State(repo), Json(payload)).await;

        // Check the response
        assert!(response.is_err());
        let error = response.unwrap_err();
        assert_eq!(error.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_project_with_empty_description() {
        let repo = MockProjectRepository;

        // Create a test payload with an empty description
        let payload = CreateProjectPayload {
            name: "Test Project".to_string(),
            description: "".to_string(),
        };

        // Call the create_project handler
        let response = create_project(State(repo), Json(payload)).await;

        // Check the response
        assert!(response.is_err());
        let error = response.unwrap_err();
        assert_eq!(error.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_get_projects() {
        let repo = MockProjectRepository;

        // Create test parameters
        let params = GetProjectsParams {
            page: Some(1),
            page_size: Some(10),
        };

        // Call the get_projects handler
        let response = get_projects(State(repo), Query(params)).await;

        // Check the response
        assert!(response.is_ok());
        let projects = response.unwrap();

        // Verify the number of projects returned
        assert_eq!(projects.len(), 10);

        // Verify the structure of each project
        for (i, project) in projects.iter().enumerate() {
            assert_eq!(project.name, format!("Project {}", i + 1));
            assert_eq!(
                project.description,
                format!("Description for Project {}", i + 1)
            );
        }
    }

    #[tokio::test]
    async fn test_get_projects_with_zero_page_size() {
        let repo = MockProjectRepository;

        // Create test parameters with page_size set to 0
        let params = GetProjectsParams {
            page: Some(1),
            page_size: Some(0),
        };

        // Call the get_projects handler
        let response = get_projects(State(repo), Query(params)).await;

        // Check the response
        assert!(response.is_ok());
        let projects = response.unwrap();

        // Verify that no projects are returned when page_size is 0
        assert!(projects.is_empty());
    }
}
