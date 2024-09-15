// libs/db/src/repository.rs
use crate::models::project::Project;
use crate::queries::project_queries;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait ProjectRepository: Clone + Send + Sync + 'static {
    async fn create_project(&self, name: &str, description: &str) -> Result<Project>;
    async fn get_projects(&self, page: Option<i32>, page_size: Option<i32>)
        -> Result<Vec<Project>>;
}

#[derive(Clone)]
pub struct PgProjectRepository {
    pool: PgPool,
}

impl PgProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectRepository for PgProjectRepository {
    async fn create_project(&self, name: &str, description: &str) -> Result<Project> {
        project_queries::create_project(&self.pool, name, description).await
    }

    async fn get_projects(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<Vec<Project>> {
        project_queries::get_projects(&self.pool, page, page_size).await
    }
}
