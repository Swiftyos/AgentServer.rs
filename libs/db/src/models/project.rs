use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use serde_json;

    #[test]
    fn test_serialization() {
        let project = Project {
            id: Uuid::new_v4(),
            name: "Test Project".to_string(),
            description: Some("A project for testing".to_string()),
            created_at: Utc.with_ymd_and_hms(2024, 4, 27, 12, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2024, 4, 28, 12, 0, 0).unwrap(),
        };

        let serialized = serde_json::to_string(&project).unwrap();
        println!("Serialized Project: {}", serialized);

        let deserialized: Project = serde_json::from_str(&serialized).unwrap();
        assert_eq!(project.id, deserialized.id);
        assert_eq!(project.name, deserialized.name);
        assert_eq!(project.description, deserialized.description);
        assert_eq!(project.created_at, deserialized.created_at);
        assert_eq!(project.updated_at, deserialized.updated_at);
    }
}
