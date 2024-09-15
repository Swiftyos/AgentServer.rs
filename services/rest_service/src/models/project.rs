use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// Structs for request payloads
#[derive(Serialize, Deserialize, IntoParams, ToSchema, Debug)]
pub struct CreateProjectPayload {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema, Debug)]
pub struct GetProjectsParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_project_payload() {
        let payload = CreateProjectPayload {
            name: "Test Project".to_string(),
            description: "A test project".to_string(),
        };
        assert_eq!(payload.name, "Test Project");
        assert_eq!(payload.description, "A test project".to_string());
    }

    #[test]
    fn test_create_project_payload_traits() {
        // Test Serialize trait
        let payload = CreateProjectPayload {
            name: "Test Project".to_string(),
            description: "A test project".to_string(),
        };

        let serialized = serde_json::to_string(&payload).unwrap();
        assert_eq!(
            serialized,
            r#"{"name":"Test Project","description":"A test project"}"#
        );

        // Test Deserialize trait
        let deserialized: CreateProjectPayload = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.name, "Test Project");
        assert_eq!(deserialized.description, "A test project".to_string());

        // Test Debug trait
        let payload = CreateProjectPayload {
            name: "Debug Test".to_string(),
            description: "Testing debug output".to_string(),
        };
        let debug_output = format!("{:?}", payload);
        assert_eq!(
            debug_output,
            "CreateProjectPayload { name: \"Debug Test\", description: \"Testing debug output\" }"
        );
    }

    #[test]
    fn test_get_projects_params() {
        let params = GetProjectsParams {
            page: Some(1),
            page_size: Some(10),
        };
        assert_eq!(params.page, Some(1));
        assert_eq!(params.page_size, Some(10));
    }

    #[test]
    fn test_get_projects_params_traits() {
        // Test Serialize trait
        let params = GetProjectsParams {
            page: Some(2),
            page_size: Some(20),
        };
        let serialized = serde_json::to_string(&params).unwrap();
        assert_eq!(serialized, r#"{"page":2,"page_size":20}"#);

        // Test Deserialize trait
        let deserialized: GetProjectsParams = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.page, Some(2));
        assert_eq!(deserialized.page_size, Some(20));

        // Test Debug trait
        let debug_output = format!("{:?}", params);
        assert_eq!(
            debug_output,
            "GetProjectsParams { page: Some(2), page_size: Some(20) }"
        );
    }
}
