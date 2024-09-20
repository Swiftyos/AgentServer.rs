use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, name: Option<String>) -> Self {
        let now = Utc::now();
        let id = Uuid::new_v4();
        Self {
            id,
            email,
            name,
            created_at: now,
            updated_at: now,
        }
    }
}
