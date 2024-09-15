use async_trait::async_trait;

/// A trait representing a message broker for pub/sub operations.
///
/// This trait defines the core functionality for a message broker,
/// including topic management, message publishing, and subscription handling.
///
/// # Example
///
/// ```rust
/// use messaging::pubsub::{MessageBroker, PubSubMessage};
///
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     // Create a broker instance (implementation-specific)
///     let broker = MyMessageBroker::new("localhost:9092", "my-group-id").await;
///
///     // Create a topic
///     broker.create_topic("my-topic").await?;
///
///     // Publish a message
///     let message = PubSubMessage {
///         key: Some(b"key".to_vec()),
///         payload: b"Hello, World!".to_vec(),
///     };
///     broker.publish("my-topic", message).await?;
///
///     // Subscribe to a topic
///     broker.subscribe("my-topic", |msg| async move {
///         println!("Received message: {:?}", msg.payload);
///     }).await?;
///
///     // List topics
///     let topics = broker.list_topics().await?;
///     println!("Available topics: {:?}", topics);
///
///     // Delete a topic
///     broker.delete_topic("my-topic").await?;
///
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait MessageBroker {
    /// Creates a new topic with the given name.
    async fn create_topic(&self, topic: &str) -> Result<(), String>;

    /// Deletes the topic with the given name.
    async fn delete_topic(&self, topic: &str) -> Result<(), String>;

    /// Lists all available topics.
    async fn list_topics(&self) -> Result<Vec<String>, String>;

    /// Subscribes to a topic and processes incoming messages with the provided handler.
    ///
    /// The handler is a function that takes a `PubSubMessage` and returns a future.
    async fn subscribe<F, Fut>(&self, topic: &str, handler: F) -> Result<(), String>
    where
        F: Fn(PubSubMessage) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send;

    /// Publishes a message to the specified topic.
    async fn publish(&self, topic: &str, message: PubSubMessage) -> Result<(), String>;
}

#[derive(Clone, Debug)]
pub struct PubSubMessage {
    pub key: Option<Vec<u8>>,
    pub payload: Vec<u8>,
}
