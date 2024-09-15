use async_trait::async_trait;

#[async_trait]
pub trait MessageBroker {
    async fn create_topic(&self, topic: &str) -> Result<(), String>;
    async fn delete_topic(&self, topic: &str) -> Result<(), String>;
    async fn list_topics(&self) -> Result<Vec<String>, String>;

    async fn subscribe<F, Fut>(&self, topic: &str, handler: F) -> Result<(), String>
    where
        F: Fn(PubSubMessage) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send;

    async fn publish(&self, topic: &str, message: PubSubMessage) -> Result<(), String>;
}

#[derive(Clone)]
pub struct PubSubMessage {
    pub key: Option<Vec<u8>>,
    pub payload: Vec<u8>,
}
