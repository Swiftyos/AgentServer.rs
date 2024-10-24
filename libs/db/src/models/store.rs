use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StoreListing {
    #[serde(rename = "agentName")]
    pub agent_name: Option<String>,
    #[serde(rename = "creatorName")]
    pub creator_name: Option<String>,
    pub description: Option<String>,
    pub runs: Option<i64>,
    pub rating: Option<f64>,
    #[serde(rename = "avatarSrc", skip_serializing_if = "Option::is_none")]
    pub avatar_src: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "mediaUrls", skip_serializing_if = "Option::is_none")]
    pub media_urls: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json;

    #[test]
    fn test_serialization() {
        let store_listing = StoreListing {
            agent_name: Some("Test Agent".to_string()),
            creator_name: Some("Test Creator".to_string()),
            description: Some("A test agent".to_string()),
            runs: Some(100),
            rating: Some(4.5),
            avatar_src: Some("https://example.com/avatar.png".to_string()),
            categories: Some(vec!["AI".to_string(), "Testing".to_string()]),
            last_updated: Some(Utc::now().naive_utc()),
            version: Some("1.0.0".to_string()),
            media_urls: Some(vec!["https://example.com/media1.png".to_string()]),
        };

        let serialized = serde_json::to_string(&store_listing).unwrap();
        println!("Serialized StoreListing: {}", serialized);

        let deserialized: StoreListing = serde_json::from_str(&serialized).unwrap();
        assert_eq!(store_listing.agent_name, deserialized.agent_name);
        assert_eq!(store_listing.creator_name, deserialized.creator_name);
        assert_eq!(store_listing.description, deserialized.description);
        assert_eq!(store_listing.runs, deserialized.runs);
        assert_eq!(store_listing.rating, deserialized.rating);
        assert_eq!(store_listing.avatar_src, deserialized.avatar_src);
        assert_eq!(store_listing.categories, deserialized.categories);
        assert_eq!(store_listing.last_updated, deserialized.last_updated);
        assert_eq!(store_listing.version, deserialized.version);
        assert_eq!(store_listing.media_urls, deserialized.media_urls);
    }

    #[test]
    fn test_serialization_without_optional_fields() {
        let store_listing = StoreListing {
            agent_name: Some("Minimal Agent".to_string()),
            creator_name: Some("Minimal Creator".to_string()),
            description: Some("A minimal agent".to_string()),
            runs: Some(0),
            rating: Some(0.0),
            avatar_src: None,
            categories: None,
            last_updated: None,
            version: None,
            media_urls: None,
        };

        let serialized = serde_json::to_string(&store_listing).unwrap();
        println!("Serialized Minimal StoreListing: {}", serialized);

        let deserialized: StoreListing = serde_json::from_str(&serialized).unwrap();
        assert_eq!(store_listing.agent_name, deserialized.agent_name);
        assert_eq!(store_listing.creator_name, deserialized.creator_name);
        assert_eq!(store_listing.description, deserialized.description);
        assert_eq!(store_listing.runs, deserialized.runs);
        assert_eq!(store_listing.rating, deserialized.rating);
        assert_eq!(store_listing.avatar_src, deserialized.avatar_src);
        assert_eq!(store_listing.categories, deserialized.categories);
        assert_eq!(store_listing.last_updated, deserialized.last_updated);
        assert_eq!(store_listing.version, deserialized.version);
        assert_eq!(store_listing.media_urls, deserialized.media_urls);
    }

    #[test]
    fn test_store_listing_creation() {
        let agent_name = "New Agent".to_string();
        let creator_name = "New Creator".to_string();
        let description = "A new agent".to_string();
        let runs = 50;
        let rating = 5.0;

        let store_listing = StoreListing {
            agent_name: Some(agent_name.clone()),
            creator_name: Some(creator_name.clone()),
            description: Some(description.clone()),
            runs: Some(runs),
            rating: Some(rating),
            avatar_src: None,
            categories: None,
            last_updated: None,
            version: None,
            media_urls: None,
        };

        assert_eq!(store_listing.agent_name, Some(agent_name));
        assert_eq!(store_listing.creator_name, Some(creator_name));
        assert_eq!(store_listing.description, Some(description));
        assert_eq!(store_listing.runs, Some(runs));
        assert_eq!(store_listing.rating, Some(rating));
        assert!(store_listing.avatar_src.is_none());
        assert!(store_listing.categories.is_none());
        assert!(store_listing.last_updated.is_none());
        assert!(store_listing.version.is_none());
        assert!(store_listing.media_urls.is_none());
    }
}
