use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "UserGroupRole")]
pub enum UserGroupRole {
    #[serde(rename = "MEMBER")]
    Member,
    #[serde(rename = "OWNER")]
    Owner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "AgentExecutionStatus")]
pub enum AgentExecutionStatus {
    #[serde(rename = "INCOMPLETE")]
    Incomplete,
    #[serde(rename = "QUEUED")]
    Queued,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "FAILED")]
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "ExecutionTriggerType")]
pub enum ExecutionTriggerType {
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "SCHEDULE")]
    Schedule,
    #[serde(rename = "WEBHOOK")]
    Webhook,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "HttpMethod")]
pub enum HttpMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "PATCH")]
    Patch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "UserBlockCreditType")]
pub enum UserBlockCreditType {
    #[serde(rename = "TOP_UP")]
    TopUp,
    #[serde(rename = "USAGE")]
    Usage,
    #[serde(rename = "COMMISSION")]
    Commission,
    #[serde(rename = "PURCHASE")]
    Purchase,
    #[serde(rename = "SALE")]
    Sale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "SubmissionStatus")]
pub enum SubmissionStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "REJECTED")]
    Rejected,
}

// User Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

// UserGroup Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserGroup {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    #[serde(rename = "groupIconUrl")]
    #[sqlx(rename = "groupIconUrl")]
    pub group_icon_url: Option<String>,
}

// UserGroupMembership Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserGroupMembership {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "userId")]
    #[sqlx(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userGroupId")]
    #[sqlx(rename = "userGroupId")]
    pub user_group_id: String,
    pub role: UserGroupRole,
}

// AgentGraph Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AgentGraph {
    pub id: String,
    pub version: i32,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "createdByUserId")]
    #[sqlx(rename = "createdByUserId")]
    pub created_by_user_id: String,
    #[serde(rename = "groupId")]
    #[sqlx(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "agentGraphParentId")]
    #[sqlx(rename = "agentGraphParentId")]
    pub agent_graph_parent_id: Option<String>,
    #[serde(rename = "agentGraphParentVersion")]
    #[sqlx(rename = "agentGraphParentVersion")]
    pub agent_graph_parent_version: Option<i32>,
}

// AgentPreset Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AgentPreset {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    #[serde(rename = "isActive")]
    #[sqlx(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "userId")]
    #[sqlx(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "agentId")]
    #[sqlx(rename = "agentId")]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[sqlx(rename = "agentVersion")]
    pub agent_version: i32,
}

// UserAgents Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserAgents {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "userId")]
    #[sqlx(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "agentId")]
    #[sqlx(rename = "agentId")]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[sqlx(rename = "agentVersion")]
    pub agent_version: i32,
    #[serde(rename = "configuredAgentId")]
    #[sqlx(rename = "configuredAgentId")]
    pub configured_agent_id: Option<String>,
    #[serde(rename = "isFavorite")]
    #[sqlx(rename = "isFavorite")]
    pub is_favorite: bool,
    #[serde(rename = "isCreatedByUser")]
    #[sqlx(rename = "isCreatedByUser")]
    pub is_created_by_user: bool,
    #[serde(rename = "isPublished")]
    #[sqlx(rename = "isPublished")]
    pub is_published: bool,
    #[serde(rename = "isPublic")]
    #[sqlx(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "isArchived")]
    #[sqlx(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "isDeleted")]
    #[sqlx(rename = "isDeleted")]
    pub is_deleted: bool,
}

// AgentNode Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AgentNode {
    pub id: String,
    #[serde(rename = "agentBlockId")]
    #[sqlx(rename = "agentBlockId")]
    pub agent_block_id: String,
    #[serde(rename = "agentGraphId")]
    #[sqlx(rename = "agentGraphId")]
    pub agent_graph_id: String,
    #[serde(rename = "agentGraphVersion")]
    #[sqlx(rename = "agentGraphVersion")]
    pub agent_graph_version: i32,
    #[serde(rename = "constantInput")]
    #[sqlx(rename = "constantInput")]
    pub constant_input: Value,
    pub metadata: Value,
}

// AgentNodeLink Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AgentNodeLink {
    pub id: String,
    #[serde(rename = "agentNodeSourceId")]
    #[sqlx(rename = "agentNodeSourceId")]
    pub agent_node_source_id: String,
    #[serde(rename = "sourceName")]
    #[sqlx(rename = "sourceName")]
    pub source_name: String,
    #[serde(rename = "agentNodeSinkId")]
    #[sqlx(rename = "agentNodeSinkId")]
    pub agent_node_sink_id: String,
    #[serde(rename = "sinkName")]
    #[sqlx(rename = "sinkName")]
    pub sink_name: String,
    #[serde(rename = "isStatic")]
    #[sqlx(rename = "isStatic")]
    pub is_static: bool,
}

// AgentBlock Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AgentBlock {
    pub id: String,
    pub name: String,
    #[serde(rename = "inputSchema")]
    #[sqlx(rename = "inputSchema")]
    pub input_schema: Value,
    #[serde(rename = "outputSchema")]
    #[sqlx(rename = "outputSchema")]
    pub output_schema: Value,
}

// AgentGraphExecution Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AgentGraphExecution {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "startedAt")]
    #[sqlx(rename = "startedAt")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(rename = "executionTriggerType")]
    #[sqlx(rename = "executionTriggerType")]
    pub execution_trigger_type: ExecutionTriggerType,
    #[serde(rename = "executionStatus")]
    #[sqlx(rename = "executionStatus")]
    pub execution_status: AgentExecutionStatus,
    #[serde(rename = "agentGraphId")]
    #[sqlx(rename = "agentGraphId")]
    pub agent_graph_id: String,
    #[serde(rename = "agentGraphVersion")]
    #[sqlx(rename = "agentGraphVersion")]
    pub agent_graph_version: i32,
    #[serde(rename = "executedByUserId")]
    #[sqlx(rename = "executedByUserId")]
    pub executed_by_user_id: String,
    pub stats: Option<Value>,
}

// AgentNodeExecution Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AgentNodeExecution {
    pub id: String,
    #[serde(rename = "agentGraphExecutionId")]
    #[sqlx(rename = "agentGraphExecutionId")]
    pub agent_graph_execution_id: String,
    #[serde(rename = "agentNodeId")]
    #[sqlx(rename = "agentNodeId")]
    pub agent_node_id: String,
    #[serde(rename = "executionStatus")]
    #[sqlx(rename = "executionStatus")]
    pub execution_status: AgentExecutionStatus,
    #[serde(rename = "executionData")]
    #[sqlx(rename = "executionData")]
    pub execution_data: Option<String>,
    #[serde(rename = "addedTime")]
    #[sqlx(rename = "addedTime")]
    pub added_time: DateTime<Utc>,
    #[serde(rename = "queuedTime")]
    #[sqlx(rename = "queuedTime")]
    pub queued_time: Option<DateTime<Utc>>,
    #[serde(rename = "startedTime")]
    #[sqlx(rename = "startedTime")]
    pub started_time: Option<DateTime<Utc>>,
    #[serde(rename = "endedTime")]
    #[sqlx(rename = "endedTime")]
    pub ended_time: Option<DateTime<Utc>>,
    pub stats: Option<Value>,
}

// AgentNodeExecutionInputOutput Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AgentNodeExecutionInputOutput {
    pub id: String,
    pub name: String,
    pub data: String,
    pub time: DateTime<Utc>,
    #[serde(rename = "referencedByInputExecId")]
    #[sqlx(rename = "referencedByInputExecId")]
    pub referenced_by_input_exec_id: Option<String>,
    #[serde(rename = "referencedByOutputExecId")]
    #[sqlx(rename = "referencedByOutputExecId")]
    pub referenced_by_output_exec_id: Option<String>,
    #[serde(rename = "configuredAgentId")]
    #[sqlx(rename = "configuredAgentId")]
    pub configured_agent_id: Option<String>,
}

// AgentGraphExecutionSchedule Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AgentGraphExecutionSchedule {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "configuredAgentId")]
    #[sqlx(rename = "configuredAgentId")]
    pub configured_agent_id: String,
    pub schedule: String,
    #[serde(rename = "isEnabled")]
    #[sqlx(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "triggerIdentifier")]
    #[sqlx(rename = "triggerIdentifier")]
    pub trigger_identifier: String,
    #[serde(rename = "lastUpdated")]
    #[sqlx(rename = "lastUpdated")]
    pub last_updated: DateTime<Utc>,
    #[serde(rename = "userId")]
    #[sqlx(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "agentId")]
    #[sqlx(rename = "agentId")]
    pub agent_id: Option<String>,
    #[serde(rename = "agentVersion")]
    #[sqlx(rename = "agentVersion")]
    pub agent_version: Option<i32>,
}

// WebhookTrigger Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WebhookTrigger {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "configuredAgentId")]
    #[sqlx(rename = "configuredAgentId")]
    pub configured_agent_id: String,
    pub method: HttpMethod,
    #[serde(rename = "urlSlug")]
    #[sqlx(rename = "urlSlug")]
    pub url_slug: String,
    #[serde(rename = "triggerIdentifier")]
    #[sqlx(rename = "triggerIdentifier")]
    pub trigger_identifier: String,
    #[serde(rename = "isActive")]
    #[sqlx(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "lastReceivedDataAt")]
    #[sqlx(rename = "lastReceivedDataAt")]
    pub last_received_data_at: Option<DateTime<Utc>>,
    #[serde(rename = "isDeleted")]
    #[sqlx(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "agentId")]
    #[sqlx(rename = "agentId")]
    pub agent_id: Option<String>,
    #[serde(rename = "agentVersion")]
    #[sqlx(rename = "agentVersion")]
    pub agent_version: Option<i32>,
}

// AnalyticsDetails Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AnalyticsDetails {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "userId")]
    #[sqlx(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub detail_type: String,
    pub data: Option<Value>,
    #[serde(rename = "dataIndex")]
    #[sqlx(rename = "dataIndex")]
    pub data_index: Option<String>,
}

// AnalyticsMetrics Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AnalyticsMetrics {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "analyticMetric")]
    #[sqlx(rename = "analyticMetric")]
    pub analytic_metric: String,
    pub value: f64,
    #[serde(rename = "dataString")]
    #[sqlx(rename = "dataString")]
    pub data_string: Option<String>,
    #[serde(rename = "userId")]
    #[sqlx(rename = "userId")]
    pub user_id: String,
}

// UserBlockCredit Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserBlockCredit {
    #[serde(rename = "transactionKey")]
    #[sqlx(rename = "transactionKey")]
    pub transaction_key: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "userId")]
    #[sqlx(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "blockId")]
    #[sqlx(rename = "blockId")]
    pub block_id: Option<String>,
    #[serde(rename = "executedAgentId")]
    #[sqlx(rename = "executedAgentId")]
    pub executed_agent_id: Option<String>,
    #[serde(rename = "executedAgentVersion")]
    #[sqlx(rename = "executedAgentVersion")]
    pub executed_agent_version: Option<i32>,
    #[serde(rename = "storeListingId")]
    #[sqlx(rename = "storeListingId")]
    pub store_listing_id: Option<String>,
    pub amount: i32,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub credit_type: UserBlockCreditType,
    #[serde(rename = "isActive")]
    #[sqlx(rename = "isActive")]
    pub is_active: bool,
    pub metadata: Option<Value>,
    #[serde(rename = "userAccountingId")]
    #[sqlx(rename = "userAccountingId")]
    pub user_accounting_id: Option<String>,
}

// UserAccounting Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserAccounting {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "userId")]
    #[sqlx(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "usdBalance")]
    #[sqlx(rename = "usdBalance")]
    pub usd_balance: f64,
    #[serde(rename = "subscriptionId")]
    #[sqlx(rename = "subscriptionId")]
    pub subscription_id: Option<String>,
}

// UserSubscription Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserSubscription {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "userId")]
    #[sqlx(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "isSubscribed")]
    #[sqlx(rename = "isSubscribed")]
    pub is_subscribed: bool,
    #[serde(rename = "subscriptionStartDate")]
    #[sqlx(rename = "subscriptionStartDate")]
    pub subscription_start_date: Option<DateTime<Utc>>,
    #[serde(rename = "subscriptionEndDate")]
    #[sqlx(rename = "subscriptionEndDate")]
    pub subscription_end_date: Option<DateTime<Utc>>,
    #[serde(rename = "isCancelling")]
    #[sqlx(rename = "isCancelling")]
    pub is_cancelling: bool,
    #[serde(rename = "subscriptionPlanId")]
    #[sqlx(rename = "subscriptionPlanId")]
    pub subscription_plan_id: String,
    #[serde(rename = "stripeCustomerId")]
    #[sqlx(rename = "stripeCustomerId")]
    pub stripe_customer_id: Option<String>,
    #[serde(rename = "stripeSubscriptionId")]
    #[sqlx(rename = "stripeSubscriptionId")]
    pub stripe_subscription_id: Option<String>,
    #[serde(rename = "stripePaymentMethodId")]
    #[sqlx(rename = "stripePaymentMethodId")]
    pub stripe_payment_method_id: Option<String>,
    #[serde(rename = "stripeSubscriptionStatus")]
    #[sqlx(rename = "stripeSubscriptionStatus")]
    pub stripe_subscription_status: Option<String>,
    #[serde(rename = "currentPeriodEnd")]
    #[sqlx(rename = "currentPeriodEnd")]
    pub current_period_end: Option<DateTime<Utc>>,
    #[serde(rename = "hasInitiatedChargeBackProcess")]
    #[sqlx(rename = "hasInitiatedChargeBackProcess")]
    pub has_initiated_charge_back_process: bool,
    #[serde(rename = "chargeBackProcessInitiatedAt")]
    #[sqlx(rename = "chargeBackProcessInitiatedAt")]
    pub charge_back_process_initiated_at: Option<DateTime<Utc>>,
}

// SubscriptionPlan Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SubscriptionPlan {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    #[serde(rename = "planIconUrl")]
    #[sqlx(rename = "planIconUrl")]
    pub plan_icon_url: Option<String>,
    #[serde(rename = "creditsPerMonth")]
    #[sqlx(rename = "creditsPerMonth")]
    pub credits_per_month: i32,
    #[serde(rename = "usdPrice")]
    #[sqlx(rename = "usdPrice")]
    pub usd_price: f64,
    #[serde(rename = "stripePriceId")]
    #[sqlx(rename = "stripePriceId")]
    pub stripe_price_id: Option<String>,
    #[serde(rename = "isDeleted")]
    #[sqlx(rename = "isDeleted")]
    pub is_deleted: bool,
}

// StripeEvent Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StripeEvent {
    pub id: String,
    #[serde(rename = "eventId")]
    #[sqlx(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "eventType")]
    #[sqlx(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "processedAt")]
    #[sqlx(rename = "processedAt")]
    pub processed_at: Option<DateTime<Utc>>,
}

// Profile Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Profile {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "userId")]
    #[sqlx(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "isGroupProfile")]
    #[sqlx(rename = "isGroupProfile")]
    pub is_group_profile: bool,
    #[serde(rename = "groupId")]
    #[sqlx(rename = "groupId")]
    pub group_id: Option<String>,
    pub username: String,
    pub description: String,
    pub links: Option<Vec<String>>,
    #[serde(rename = "avatarUrl")]
    #[sqlx(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
}

// StoreListing Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StoreListing {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "isDeleted")]
    #[sqlx(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "isApproved")]
    #[sqlx(rename = "isApproved")]
    pub is_approved: bool,
    pub slug: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "agentId")]
    #[sqlx(rename = "agentId")]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[sqlx(rename = "agentVersion")]
    pub agent_version: i32,
    #[serde(rename = "owningUserId")]
    #[sqlx(rename = "owningUserId")]
    pub owning_user_id: String,
    #[serde(rename = "isGroupListing")]
    #[sqlx(rename = "isGroupListing")]
    pub is_group_listing: bool,
    #[serde(rename = "owningGroupId")]
    #[sqlx(rename = "owningGroupId")]
    pub owning_group_id: Option<String>,
}

// StorePricing Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StorePricing {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "storeListingId")]
    #[sqlx(rename = "storeListingId")]
    pub store_listing_id: String,
    #[serde(rename = "canPurchase")]
    #[sqlx(rename = "canPurchase")]
    pub can_purchase: bool,
    #[serde(rename = "purchasePrice")]
    #[sqlx(rename = "purchasePrice")]
    pub purchase_price: Option<f64>,
    #[serde(rename = "commissionPerRun")]
    #[sqlx(rename = "commissionPerRun")]
    pub commission_per_run: Option<f64>,
    #[serde(rename = "isDeleted")]
    #[sqlx(rename = "isDeleted")]
    pub is_deleted: bool,
}

// StoreListingVersion Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StoreListingVersion {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "agentId")]
    #[sqlx(rename = "agentId")]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[sqlx(rename = "agentVersion")]
    pub agent_version: i32,
    #[serde(rename = "isFeatured")]
    #[sqlx(rename = "isFeatured")]
    pub is_featured: bool,
    pub categories: Option<Vec<String>>,
    #[serde(rename = "isDeleted")]
    #[sqlx(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "isAvailable")]
    #[sqlx(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "isApproved")]
    #[sqlx(rename = "isApproved")]
    pub is_approved: bool,
    #[serde(rename = "storeListingId")]
    #[sqlx(rename = "storeListingId")]
    pub store_listing_id: Option<String>,
}

// StoreListingSubmission Table
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StoreListingSubmission {
    pub id: String,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "storeListingId")]
    #[sqlx(rename = "storeListingId")]
    pub store_listing_id: String,
    #[serde(rename = "storeListingVersionId")]
    #[sqlx(rename = "storeListingVersionId")]
    pub store_listing_version_id: String,
    #[serde(rename = "reviewByUserId")]
    #[sqlx(rename = "reviewByUserId")]
    pub review_by_user_id: String,
    pub status: SubmissionStatus,
    #[serde(rename = "isDenied")]
    #[sqlx(rename = "isDenied")]
    pub is_denied: bool,
    #[serde(rename = "reviewComments")]
    #[sqlx(rename = "reviewComments")]
    pub review_comments: Option<String>,
}
