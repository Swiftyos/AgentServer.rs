use crate::models::StoreListing;
use anyhow::Result;
use sqlx::PgPool;
use tracing::{info, instrument};

/// Retrieves store listings with aggregated data from the database.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `page` - Optional page number for pagination (1-indexed).
/// * `page_size` - Optional number of items per page.
///
/// # Returns
///
/// * `Result<Vec<StoreListing>>` - A Result containing a vector of StoreListing structs if successful,
///   or an error if the query fails.
#[instrument(name = "db.get_store_listings", skip_all, fields(page, page_size))]
pub async fn get_store_listings(
    pool: &PgPool,
    page: Option<i32>,
    page_size: Option<i32>,
) -> Result<Vec<StoreListing>> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    info!(
        "Fetching store listings with page: {:?}, page_size: {:?}",
        page, page_size
    );

    let listings = sqlx::query_as!(
        StoreListing,
        r#"
        WITH ReviewStats AS (
            SELECT 
                sr."storeListingId",
                COUNT(*) as review_count,
                COALESCE(AVG(CAST(sr.score AS DECIMAL)), 0.0) as avg_rating
            FROM "StoreListingReview" sr
            GROUP BY sr."storeListingId"
        )
        SELECT 
            a.name as agent_name,
            COALESCE(p.username, u.name, 'Unknown') as creator_name,
            sl.description,
            COALESCE(ae.run_count, 0) as runs,
            CAST(COALESCE(rs.avg_rating, 0.0) AS DOUBLE PRECISION) as rating,
            p."avatarUrl" as avatar_src,
            slv.categories,
            sl."updatedAt" as last_updated,
            CAST(a.version AS TEXT) as version,
            COALESCE(sl."mediaUrls", ARRAY[]::TEXT[]) as media_urls
        FROM "StoreListing" sl
        LEFT JOIN "Agent" a ON sl."agentId" = a.id AND sl."agentVersion" = a.version
        LEFT JOIN "User" u ON sl."owningUserId" = u.id
        LEFT JOIN "Profile" p ON u.id = p."userId"
        LEFT JOIN ReviewStats rs ON sl.id = rs."storeListingId"
        LEFT JOIN "StoreListingVersion" slv ON sl.id = slv."storeListingId"
        LEFT JOIN (
            SELECT "agentId", COUNT(*) as run_count 
            FROM "AgentExecution"
            GROUP BY "agentId"
        ) ae ON a.id = ae."agentId"
        WHERE sl."isDeleted" = false
          AND sl."isApproved" = true
        ORDER BY sl."updatedAt" DESC
        LIMIT $1 OFFSET $2
        "#,
        page_size as i64,
        offset as i64
    )
    .fetch_all(pool)
    .await?;

    info!("Fetched {} store listings", listings.len());

    Ok(listings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::{apply_migrations, create_pool};
    use config::{Config, Environment, File};
    use sqlx::{Pool, Postgres};
    use tracing::{error, info};
    use tracing_test::traced_test;
    use uuid::Uuid;

    async fn setup_db() -> Pool<Postgres> {
        let config = Config::builder()
            .add_source(File::with_name("../../config/test.toml"))
            .add_source(Environment::with_prefix("APP"))
            .build()
            .expect("Failed to load configuration");

        let database_url = config
            .get_string("database_url")
            .expect("DATABASE_URL must be set in config");

        let schema_string = format!(
            "test_schema_{}",
            uuid::Uuid::new_v4().to_string().replace("-", "")
        );
        let schema = Some(schema_string.as_str());

        info!("Database URL: {}", database_url);
        info!("Schema: {}", schema_string);
        let pool = create_pool(&database_url, schema)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create database pool: {}", e));
        match pool {
            Ok(pool) => {
                apply_migrations(&pool).await.unwrap();
                pool
            }
            Err(e) => {
                error!("Failed to create database pool: {}", e);
                panic!("Failed to create database pool: {}", e);
            }
        }
    }

    #[traced_test]
    #[tokio::test]
    async fn test_get_store_listings() {
        let pool = setup_db().await;

        // Clean up relevant tables
        sqlx::query(r#"TRUNCATE TABLE "StoreListing", "Agent", "User", "Profile", "StoreListingReview", "StoreListingVersion", "AgentExecution", "StoreListingSubmission" CASCADE"#)
            .execute(&pool)
            .await
            .unwrap();

        let test_user_id = Uuid::new_v4();
        let test_agent_id = Uuid::new_v4();
        let test_listing_id = Uuid::new_v4();

        // Insert test data
        // First create a user
        sqlx::query(
            r#"
            INSERT INTO "User" (id, name, email, metadata) 
            VALUES ($1, 'Test User', 'test@example.com',  '{}'::jsonb)
        "#,
        )
        .bind(test_user_id)
        .execute(&pool)
        .await
        .unwrap();

        // Create a profile for the user
        sqlx::query(r#"
            INSERT INTO "Profile" (id, "userId", "isGroupProfile", username, description, links, "avatarUrl") 
            VALUES (gen_random_uuid(), $1, false, 'testuser', '', ARRAY[]::text[], 'https://example.com/avatar.png')
        "#)
        .bind(test_user_id)
        .execute(&pool)
        .await
        .unwrap();

        // Create an agent
        sqlx::query(r#"
            INSERT INTO "Agent" (id, version, name, description, "createdByUserId", "groupId", "agentParentId", "agentParentVersion")
            VALUES ($1, 1,  'Test Agent', NULL, NULL, NULL, NULL, NULL)
        "#)
        .bind(test_agent_id)
        .execute(&pool)
        .await
        .unwrap();

        // Create a store listing
        sqlx::query(r#"
            INSERT INTO "StoreListing" (id, "isDeleted", "isApproved", slug, name, description, "agentId", "agentVersion", "owningUserId", "isGroupListing", "owningGroupId")
            VALUES ($1, false, true, 'test-listing', 'Test Listing', 'Test Description', $2, 1, $3, false, NULL)
        "#)
        .bind(test_listing_id)
        .bind(test_agent_id)
        .bind(test_user_id)
        .execute(&pool)
        .await
        .unwrap();

        // Create a store listing version
        sqlx::query(r#"
            INSERT INTO "StoreListingVersion" (id, "agentId", "agentVersion", "isFeatured", categories, "isDeleted", "isAvailable", "isApproved", "storeListingId")
            VALUES (gen_random_uuid(), $1, 1, false, ARRAY['AI', 'Testing'], false, true, true, $2)
        "#)
        .bind(test_agent_id)
        .bind(test_listing_id)
        .execute(&pool)
        .await
        .unwrap();

        // Test get_store_listings
        let listings = get_store_listings(&pool, Some(1), Some(10)).await.unwrap();

        assert_eq!(listings.len(), 1);
        let listing = &listings[0];
        assert_eq!(listing.agent_name, Some("Test Agent".to_string()));
        assert_eq!(listing.creator_name, Some("testuser".to_string()));
        assert_eq!(listing.description, Some("Test Description".to_string()));
        assert_eq!(
            listing.avatar_src,
            Some("https://example.com/avatar.png".to_string())
        );
        assert_eq!(
            listing.categories,
            Some(vec!["AI".to_string(), "Testing".to_string()])
        );
        assert_eq!(listing.media_urls, Some(vec![]));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_store_listings_pagination() {
        let pool = setup_db().await;

        // Clean up relevant tables
        sqlx::query(r#"TRUNCATE TABLE "StoreListing", "Agent", "User", "Profile", "StoreListingReview", "StoreListingVersion", "AgentExecution" CASCADE"#)
            .execute(&pool)
            .await
            .unwrap();

        let test_user_id = Uuid::new_v4();
        let test_agent_id = Uuid::new_v4();

        // Create test user and agent first
        sqlx::query(
            r#"
            INSERT INTO "User" (id, name, email) 
            VALUES ($1, 'Test User', 'test@example.com')
        "#,
        )
        .bind(test_user_id)
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            INSERT INTO "Agent" (id, name, version)
            VALUES ($1, 'Test Agent', 1)
        "#,
        )
        .bind(test_agent_id)
        .execute(&pool)
        .await
        .unwrap();

        // Create multiple store listings
        for i in 1..=15 {
            let listing_id = Uuid::new_v4();
            sqlx::query(
                r#"
                INSERT INTO "StoreListing" (
                    id,
                    "owningUserId",
                    "agentId",
                    "agentVersion",
                    name,
                    slug,
                    description,
                    "isDeleted",
                    "isApproved"
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, false, true)
            "#,
            )
            .bind(listing_id)
            .bind(test_user_id)
            .bind(test_agent_id)
            .bind(1)
            .bind(format!("Test Agent {}", i))
            .bind(format!("test-agent-{}", i))
            .bind(format!("Description {}", i))
            .execute(&pool)
            .await
            .unwrap();
        }

        // Test first page
        let first_page = get_store_listings(&pool, Some(1), Some(10)).await.unwrap();
        assert_eq!(
            first_page.len(),
            10,
            "First page should contain 10 listings"
        );

        // Test second page
        let second_page = get_store_listings(&pool, Some(2), Some(10)).await.unwrap();
        assert_eq!(
            second_page.len(),
            5,
            "Second page should contain 5 listings"
        );

        // Test empty page
        let empty_page = get_store_listings(&pool, Some(3), Some(10)).await.unwrap();
        assert_eq!(empty_page.len(), 0, "Third page should be empty");
    }
}
