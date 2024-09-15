use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
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
            description: "A project for testing".to_string(),
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

    #[test]
    fn test_serialization_without_optional_fields() {
        let project = Project {
            id: Uuid::new_v4(),
            name: "Minimal Project".to_string(),
            description: "Some description".to_string(),
            created_at: Utc.with_ymd_and_hms(2024, 5, 1, 9, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2024, 5, 1, 9, 0, 0).unwrap(),
        };

        let serialized = serde_json::to_string(&project).unwrap();
        println!("Serialized Minimal Project: {}", serialized);

        let deserialized: Project = serde_json::from_str(&serialized).unwrap();
        assert_eq!(project.id, deserialized.id);
        assert_eq!(project.name, deserialized.name);
        assert_eq!(project.description, deserialized.description);
        assert_eq!(project.created_at, deserialized.created_at);
        assert_eq!(project.updated_at, deserialized.updated_at);
    }

    #[test]
    fn test_project_creation() {
        let id = Uuid::new_v4();
        let name = "New Project".to_string();
        let description = "A newly created project".to_string();
        let created_at = Utc::now();
        let updated_at = Utc::now();

        let project = Project {
            id,
            name: name.clone(),
            description: description.clone(),
            created_at,
            updated_at,
        };

        assert_eq!(project.id, id);
        assert_eq!(project.name, name);
        assert_eq!(project.description, description);
        assert_eq!(project.created_at, created_at);
        assert_eq!(project.updated_at, updated_at);
    }
}
