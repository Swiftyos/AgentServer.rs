// libs/db/src/repository.rs
use crate::models::project::Project;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ProjectRepository: Clone + Send + Sync + 'static {
    async fn create_project(&self, name: &str, description: &str) -> Result<Project>;
    async fn get_projects(&self, page: Option<i32>, page_size: Option<i32>)
        -> Result<Vec<Project>>;
}
