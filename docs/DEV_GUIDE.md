# Developer Guide: Adding a New API Route and Database Table

This guide demonstrates how to apply the best practices outlined in our [Contributing Guide](CONTRIBUTING.md) by walking you through the process of adding a new API route and a corresponding database table with queries. We'll cover the entire workflow, from planning to submitting a pull request, ensuring that the new feature is bug-free, efficient, and adheres to our project's standards.

---

## Table of Contents

1. [Introduction](#introduction)
2. [Scenario Overview](#scenario-overview)
3. [Step 1: Planning and Issue Tracking](#step-1-planning-and-issue-tracking)
4. [Step 2: Setting Up the Development Environment](#step-2-setting-up-the-development-environment)
5. [Step 3: Creating a Feature Branch](#step-3-creating-a-feature-branch)
6. [Step 4: Modifying the Database Schema](#step-4-modifying-the-database-schema)
   - [Adding a New Migration](#adding-a-new-migration)
   - [Updating Database Models](#updating-database-models)
7. [Step 5: Implementing Database Queries and Repository](#step-5-implementing-database-queries-and-repository)
8. [Step 6: Updating Shared Libraries](#step-6-updating-shared-libraries)
9. [Step 7: Implementing the API Route in the REST Service](#step-7-implementing-the-api-route-in-the-rest-service)
   - [Adding Route Handlers](#adding-route-handlers)
   - [Registering the Route](#registering-the-route)
10. [Step 8: Writing Tests](#step-8-writing-tests)
    - [Unit Tests with Mocked Database](#unit-tests-with-mocked-database)
    - [Integration Tests](#integration-tests)
11. [Step 9: Ensuring Code Quality](#step-9-ensuring-code-quality)
    - [Formatting and Linting](#formatting-and-linting)
    - [Static Analysis](#static-analysis)
12. [Step 10: Documentation](#step-10-documentation)
13. [Step 11: Committing and Pushing Changes](#step-11-committing-and-pushing-changes)
14. [Step 12: Submitting a Pull Request](#step-12-submitting-a-pull-request)
15. [Conclusion](#conclusion)
16. [Additional Resources](#additional-resources)

---

## Introduction

In this guide, we'll add a new feature to the **REST service**: an endpoint to manage "Projects". We'll create a new database table for projects, update the shared libraries with new models and repositories, implement database queries, and expose API routes for creating and retrieving projects. We'll follow best practices for development, testing (including mocking the database), and code quality to ensure our contribution is efficient and reliable.

---

## Scenario Overview

- **Feature**: Add API endpoints to create and retrieve projects.
- **Database Changes**: Introduce a new `projects` table.
- **Components Affected**:
  - **Libraries**:
    - `libs/db`: New models, repositories, and database interactions.
    - `libs/common`: Any shared models or utilities (if needed).
  - **Services**:
    - `services/rest_service`: New API routes and handlers.

---

## Step 1: Planning and Issue Tracking

- **Create an Issue**: Open a GitHub issue titled "Add Project Management API".
- **Define the Scope**:
  - **Endpoints**:
    - `POST /api/projects`: Create a new project.
    - `GET /api/projects`: Retrieve a list of projects.
  - **Database**:
    - New `projects` table with fields `id`, `name`, `description`, `created_at`, and `updated_at`.
- **Assign the Issue**: Assign yourself to the issue to indicate you are working on it.

---

## Step 2: Setting Up the Development Environment

- **Ensure Tools Are Installed**:
  - Rust Toolchain (`rustup`)
  - `cargo-edit`, `cargo-watch`, `cargo-nextest`, `clippy`, `rustfmt`, `cargo-audit`
- **Update Dependencies**:

  ```bash
  cd agentserver.rs
  cargo update
  ```

- **Start the Development Database**: Ensure your PostgreSQL instance is running and accessible.

---

## Step 3: Creating a Feature Branch

```bash
git checkout -b feature/project-management develop
```

---

## Step 4: Modifying the Database Schema

### Adding a New Migration

We'll use `sqlx`'s migration tool to create a new migration.

1. **Navigate to the `libs/db` Directory**:

   ```bash
   cd libs/db
   ```

2. **Create Migration File**:

   ```bash
   sqlx migrate add create_projects_table
   ```

   This creates a new SQL file in `libs/db/migrations/`.

3. **Edit Migration File**: Open the newly created migration file and add the SQL statements.

   ```sql
   -- libs/db/migrations/{timestamp}__create_projects_table.sql

   CREATE TABLE projects (
     id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
     name TEXT NOT NULL,
     description TEXT NOT NULL,
     created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
     updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
   );
   ```

4. **Apply Migrations**: Run migrations against your development database.

   ```bash
   sqlx migrate run
   ```

### Updating Database Models

1. **Create a Project Model**:

   ```rust
   // libs/db/src/models/project.rs
   use serde::{Deserialize, Serialize};
   use uuid::Uuid;
   use chrono::{DateTime, Utc};
   use sqlx::FromRow;

   #[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
   pub struct Project {
       pub id: Uuid,
       pub name: String,
       pub description: String,
       pub created_at: DateTime<Utc>,
       pub updated_at: DateTime<Utc>,
   }
   ```

2. **Update the Models Module**:

   ```rust
   // libs/db/src/models/mod.rs
   pub mod project;
   pub use project::Project;
   // ... existing models
   ```

---

## Step 5: Implementing Database Queries and Repository

To allow for mocking the database in tests, we'll implement a `ProjectRepository` trait. This abstraction lets us swap out the actual database implementation with a mock when testing.

1. **Create the Repository Trait**:

   ```rust
   // libs/db/src/repository/project_repository.rs
   use async_trait::async_trait;
   use anyhow::Result;
   use crate::models::Project;

   #[async_trait]
   pub trait ProjectRepository: Clone + Send + Sync + 'static {
       async fn create_project(&self, name: &str, description: &str) -> Result<Project>;
       async fn get_projects(&self, page: Option<i32>, page_size: Option<i32>) -> Result<Vec<Project>>;
   }
   ```

2. **Implement the Repository for Database Interactions**:

   ```rust
   // libs/db/src/repository/project_repository_impl.rs
   use super::project_repository::ProjectRepository;
   use async_trait::async_trait;
   use anyhow::Result;
   use crate::models::Project;
   use sqlx::PgPool;

   #[derive(Clone)]
   pub struct ProjectRepositoryImpl {
       pub pool: PgPool,
   }

   #[async_trait]
   impl ProjectRepository for ProjectRepositoryImpl {
       async fn create_project(&self, name: &str, description: &str) -> Result<Project> {
           let project = sqlx::query_as::<_, Project>(
               r#"
               INSERT INTO projects (name, description)
               VALUES ($1, $2)
               RETURNING *
               "#
           )
           .bind(name)
           .bind(description)
           .fetch_one(&self.pool)
           .await?;
           Ok(project)
       }

       async fn get_projects(&self, page: Option<i32>, page_size: Option<i32>) -> Result<Vec<Project>> {
           let page = page.unwrap_or(1);
           let page_size = page_size.unwrap_or(10);
           let offset = (page - 1) * page_size;

           let projects = sqlx::query_as::<_, Project>(
               r#"
               SELECT * FROM projects
               ORDER BY created_at DESC
               LIMIT $1 OFFSET $2
               "#
           )
           .bind(page_size)
           .bind(offset)
           .fetch_all(&self.pool)
           .await?;
           Ok(projects)
       }
   }
   ```

3. **Update the Repository Module**:

   ```rust
   // libs/db/src/repository/mod.rs
   pub mod project_repository;
   pub use project_repository::ProjectRepository;

   pub mod project_repository_impl;
   pub use project_repository_impl::ProjectRepositoryImpl;
   ```

- **Best Practices Applied**:
  - **Abstraction**: Using a trait to abstract database operations.
  - **Dependency Injection**: Allows for injecting different implementations, such as mocks.
  - **Async/Await and Error Handling**: As before.

---

## Step 6: Updating Shared Libraries

1. **Update `libs/db/src/lib.rs`**:

   ```rust
   // libs/db/src/lib.rs
   pub mod models;
   pub mod repository;
   pub mod connection; // If you have a connection module
   // ... other modules
   ```

2. **Ensure Proper Exports**:
   - Re-export structs or functions if necessary for easier access in services.

3. **Add Dependencies in `services/rest_service/Cargo.toml`**:

   ```toml
   [dependencies]
   db = { path = "../../libs/db" }
   common = { path = "../../libs/common" } # If needed
   async-trait = "0.1" # Needed for the repository trait
   # ... other dependencies
   ```

---

## Step 7: Implementing the API Route in the REST Service

### Adding Route Handlers

1. **Create Handler Functions**:

   ```rust
   // services/rest_service/src/handlers/projects.rs
   use axum::{
       extract::{State, Query, Json},
       http::StatusCode,
       response::{IntoResponse, Response},
   };
   use db::repository::ProjectRepository;
   use db::models::Project;
   use serde::{Deserialize, Serialize};
   use tracing::instrument;
   use anyhow::Result;

   #[derive(Deserialize)]
   pub struct CreateProjectPayload {
       pub name: String,
       pub description: String,
   }

   #[derive(Deserialize)]
   pub struct GetProjectsParams {
       pub page: Option<i32>,
       pub page_size: Option<i32>,
   }

   #[instrument(skip(repo))]
   pub async fn create_project_handler<R: ProjectRepository>(
       State(repo): State<R>,
       Json(payload): Json<CreateProjectPayload>,
   ) -> Result<Json<Project>, Response> {
       if payload.name.trim().is_empty() {
           return Err((
               StatusCode::BAD_REQUEST,
               "Project name cannot be empty",
           )
           .into_response());
       }
       if payload.description.trim().is_empty() {
           return Err((
               StatusCode::BAD_REQUEST,
               "Project description cannot be empty",
           )
           .into_response());
       }
       let project = repo
           .create_project(&payload.name, &payload.description)
           .await
           .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
           .unwrap();
       Ok(Json(project))
   }

   #[instrument(skip(repo))]
   pub async fn get_projects_handler<R: ProjectRepository>(
       State(repo): State<R>,
       Query(params): Query<GetProjectsParams>,
   ) -> Result<Json<Vec<Project>>, StatusCode> {
       let projects = repo
           .get_projects(params.page, params.page_size)
           .await
           .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
       Ok(Json(projects))
   }
   ```

   - **Best Practices Applied**:
     - **Mocking Support**: By abstracting the repository, we can inject a mock repository in tests.
     - **Error Handling**: Converting errors into HTTP responses.
     - **Logging**: Using `tracing::instrument` to log function calls and parameters.
     - **Dependency Injection**: Using `State` to pass the repository.

2. **Update the Handlers Module**:

   ```rust
   // services/rest_service/src/handlers/mod.rs
   pub mod projects;
   // ... existing handlers
   ```

### Registering the Route

1. **Update the Routes Module**:

   ```rust
   // services/rest_service/src/routes.rs
   use axum::{
       routing::{get, post},
       Router,
   };
   use crate::handlers::projects::{create_project_handler, get_projects_handler};
   use db::repository::ProjectRepository;

   pub fn create_router<R: ProjectRepository>(repo: R) -> Router {
       Router::new()
           .route(
               "/api/projects",
               post(create_project_handler::<R>).get(get_projects_handler::<R>),
           )
           .with_state(repo)
           // ... existing routes
   }
   ```

2. **Ensure Routes Are Mounted in `main.rs`**:

   ```rust
   // services/rest_service/src/main.rs
   use axum::Router;
   use db::repository::ProjectRepositoryImpl;
   use sqlx::postgres::PgPoolOptions;
   use crate::routes::create_router;
   use tracing_subscriber;

   #[tokio::main]
   async fn main() {
       // Initialize tracing
       tracing_subscriber::fmt::init();

       // Create database pool
       let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
       let pool = PgPoolOptions::new()
           .max_connections(5)
           .connect(&database_url)
           .await
           .expect("Failed to create pool");

       // Create repository instance
       let repo = ProjectRepositoryImpl { pool };

       // Build our application with a route and repository
       let app = create_router(repo);

       // Run it
       axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
           .serve(app.into_make_service())
           .await
           .unwrap();
   }
   ```

   - **Best Practices Applied**:
     - **Dependency Injection**: Passing the repository to the router.
     - **Generic Routes**: Making the router generic over the `ProjectRepository` trait.

---

## Step 8: Writing Tests

### Unit Tests with Mocked Database

We will create unit tests for our handlers using a mock implementation of `ProjectRepository`. This allows us to test the handlers without depending on a real database.

1. **Create a Mock Repository**:

   ```rust
   // services/rest_service/src/tests/mock_project_repository.rs
   use db::repository::ProjectRepository;
   use db::models::Project;
   use async_trait::async_trait;
   use anyhow::Result;
   use uuid::Uuid;
   use chrono::Utc;

   #[derive(Clone)]
   pub struct MockProjectRepository;

   #[async_trait]
   impl ProjectRepository for MockProjectRepository {
       async fn create_project(&self, name: &str, description: &str) -> Result<Project> {
           let new_project = Project {
               id: Uuid::new_v4(),
               name: name.to_string(),
               description: description.to_string(),
               created_at: Utc::now(),
               updated_at: Utc::now(),
           };
           Ok(new_project)
       }

       async fn get_projects(&self, page: Option<i32>, page_size: Option<i32>) -> Result<Vec<Project>> {
           let page_size = page_size.unwrap_or(10) as usize;
           let mut projects = Vec::new();
           for i in 0..page_size {
               projects.push(Project {
                   id: Uuid::new_v4(),
                   name: format!("Project {}", i + 1),
                   description: format!("Description for Project {}", i + 1),
                   created_at: Utc::now(),
                   updated_at: Utc::now(),
               });
           }
           Ok(projects)
       }
   }
   ```

2. **Write Tests for Handlers**:

   ```rust
   // services/rest_service/src/handlers/projects.rs
   #[cfg(test)]
   mod tests {
       use super::*;
       use crate::tests::mock_project_repository::MockProjectRepository;
       use axum::extract::State;
       use axum::Json;
       use axum::response::Response;

       #[tokio::test]
       async fn test_create_project() {
           let repo = MockProjectRepository;
           let payload = CreateProjectPayload {
               name: "Test Project".to_string(),
               description: "A test project description".to_string(),
           };
           let response = create_project_handler(State(repo), Json(payload)).await;
           assert!(response.is_ok());
           let project = response.unwrap();
           assert_eq!(project.name, "Test Project");
           assert_eq!(project.description, "A test project description");
       }

       #[tokio::test]
       async fn test_get_projects() {
           let repo = MockProjectRepository;
           let params = GetProjectsParams {
               page: Some(1),
               page_size: Some(5),
           };
           let response = get_projects_handler(State(repo), Query(params)).await;
           assert!(response.is_ok());
           let projects = response.unwrap();
           assert_eq!(projects.len(), 5);
           for (i, project) in projects.iter().enumerate() {
               assert_eq!(project.name, format!("Project {}", i + 1));
               assert_eq!(project.description, format!("Description for Project {}", i + 1));
           }
       }
   }
   ```

   - **Best Practices Applied**:
     - **Mocking**: Using a mock repository to avoid dependency on the real database.
     - **Isolated Tests**: Tests are self-contained and do not interact with external systems.
     - **Async Testing**: Using `#[tokio::test]` for asynchronous tests.

### Integration Tests

Integration tests can still use a real database or further mock the endpoints.

1. **Create Integration Test File**:

   ```rust
   // services/rest_service/tests/projects_integration_tests.rs
   use axum::{
       body::Body,
       http::{Request, StatusCode},
   };
   use tower::ServiceExt; // for `oneshot`
   use serde_json::json;
   use db::models::Project;
   use db::repository::ProjectRepository;
   use crate::routes::create_router;
   use crate::tests::mock_project_repository::MockProjectRepository;

   #[tokio::test]
   async fn test_create_and_list_projects_api() {
       // Set up the app with mock repository
       let repo = MockProjectRepository;
       let app = create_router(repo);

       // Test POST /api/projects
       let payload = json!({
           "name": "API Test Project",
           "description": "Test Description"
       });
       let response = app
           .oneshot(
               Request::builder()
                   .method("POST")
                   .uri("/api/projects")
                   .header("Content-Type", "application/json")
                   .body(Body::from(payload.to_string()))
                   .unwrap(),
           )
           .await
           .unwrap();
       assert_eq!(response.status(), StatusCode::OK);

       // Test GET /api/projects
       let response = app
           .oneshot(
               Request::builder()
                   .method("GET")
                   .uri("/api/projects?page=1&page_size=5")
                   .body(Body::empty())
                   .unwrap(),
           )
           .await
           .unwrap();
       assert_eq!(response.status(), StatusCode::OK);
       let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
       let projects: Vec<Project> = serde_json::from_slice(&body).unwrap();
       assert_eq!(projects.len(), 5);
   }
   ```

   - **Best Practices Applied**:
     - **Integration Testing with Mocks**: Testing the full request-response cycle while using a mock repository.
     - **Using `oneshot`**: To send a request to the app without starting a server.
     - **Deserializing Responses**: To verify the correctness of API outputs.

2. **Update `Cargo.toml` for Tests**:

   Ensure that your `Cargo.toml` includes the `[dev-dependencies]` section if necessary.

   ```toml
   [dev-dependencies]
   axum = { version = "0.6", features = ["test"] }
   serde_json = "1.0"
   tokio = { version = "1", features = ["macros"] }
   ```

3. **Organize Test Modules**:

   ```rust
   // services/rest_service/src/tests/mod.rs
   pub mod mock_project_repository;
   ```

---

## Step 9: Ensuring Code Quality

### Formatting and Linting

1. **Run Rustfmt**:

   ```bash
   cargo fmt --all
   ```

2. **Run Clippy**:

   ```bash
   cargo clippy --all -- -D warnings
   ```

   - **Best Practices Applied**:
     - **Consistency**: Ensuring code adheres to style guidelines.
     - **Quality**: Catching potential issues early.

### Static Analysis

1. **Run Cargo Audit**:

   ```bash
   cargo audit
   ```

2. **Check for Outdated Dependencies**:

   ```bash
   cargo outdated
   ```

   - **Best Practices Applied**:
     - **Security**: Identifying vulnerabilities.
     - **Maintenance**: Keeping dependencies up to date.

---

## Step 10: Documentation

1. **Document Public Functions and Traits**:

   ```rust
   /// Repository trait for project-related database operations.
   #[async_trait]
   pub trait ProjectRepository: Clone + Send + Sync + 'static {
       /// Creates a new project in the database.
       ///
       /// # Arguments
       ///
       /// * `name` - The name of the project.
       /// * `description` - The description of the project.
       async fn create_project(&self, name: &str, description: &str) -> Result<Project>;

       /// Retrieves a list of projects from the database.
       ///
       /// # Arguments
       ///
       /// * `page` - The page number for pagination.
       /// * `page_size` - The number of projects per page.
       async fn get_projects(&self, page: Option<i32>, page_size: Option<i32>) -> Result<Vec<Project>>;
   }
   ```

2. **Update README**: If necessary, update any relevant sections in the `README.md`.

3. **Generate Documentation**:

   ```bash
   cargo doc --open
   ```

   - **Best Practices Applied**:
     - **Clarity**: Helping other developers understand your code.
     - **Maintainability**: Easier future updates and bug fixes.

---

## Step 11: Committing and Pushing Changes

1. **Add Changes**:

   ```bash
   git add .
   ```

2. **Write Descriptive Commit Messages**:

   ```bash
   git commit -m "feat(rest_service): add project management API endpoints"
   ```

   - **Follow Conventional Commits**:
     - **feat**: A new feature.
     - **fix**: A bug fix.
     - **docs**: Documentation changes.
     - **style**: Code style changes (formatting, missing semicolons, etc.).
     - **refactor**: Code changes that neither fix a bug nor add a feature.

3. **Push to Remote Branch**:

   ```bash
   git push origin feature/project-management
   ```

---

## Step 12: Submitting a Pull Request

1. **Open Pull Request**:
   - **Title**: `feat(rest_service): add project management API endpoints`
   - **Description**:
     - Briefly explain what changes you've made.
     - Reference the issue number (e.g., "Closes #123").
     - Highlight any important points or considerations, such as the introduction of a repository pattern and mocking in tests.
2. **Ensure CI Passes**:
   - Wait for Continuous Integration checks to pass.
   - Address any issues if they arise.
3. **Review Feedback**:
   - Respond promptly to code review comments.
   - Make necessary changes and push updates.

---

## Conclusion

By following this guide, you've successfully added a new API route and database table while adhering to our project's structure and best practices. You've utilized the recommended tools, written tests to ensure correctness (including mocking the database), and maintained high code quality throughout the process.

- **Project Structure Alignment**: Ensured that all code additions fit within the established `libs` and `services` directories.
- **Best Practices**: Followed Rust conventions and project guidelines for code quality, testing (with mocks), and documentation.
- **Abstraction and Modularity**: Used a repository pattern to abstract database interactions, facilitating testing and maintainability.
- **Rapid Development**: Structured the project to facilitate quick and efficient development.

---

## Additional Resources

- **Contributing Guide**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **Rust Book**: [The Rust Programming Language](https://doc.rust-lang.org/book/)
- **Tokio Documentation**: [Tokio Async Runtime](https://tokio.rs/tokio/tutorial)
- **Axum Documentation**: [Axum Web Framework](https://docs.rs/axum)
- **SQLx Documentation**: [SQLx Async Rust SQL crate](https://docs.rs/sqlx)
- **Project Structure Overview**: [Project Structure Documentation](PROJECT_STRUCTURE.md) *(if available)*

---

**Note**: Remember to keep your feature branch up-to-date with the `develop` branch by rebasing or merging as necessary. This ensures that your code integrates smoothly with the latest changes in the codebase.

If you have any questions or need assistance, don't hesitate to reach out to the project maintainers or open a discussion in the project's communication channels.

Happy coding!