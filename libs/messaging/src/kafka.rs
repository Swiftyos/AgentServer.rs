use crate::pubsub::{MessageBroker, PubSubMessage};
use async_trait::async_trait;
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};
pub struct KafkaBroker {
    producer: FutureProducer,
    consumer: Arc<StreamConsumer>,
    admin_client: AdminClient<rdkafka::client::DefaultClientContext>,
}

impl KafkaBroker {
    pub async fn new(brokers: &str, group_id: &str) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .create()
            .expect("Producer creation error");

        let consumer: Arc<StreamConsumer> = Arc::new(
            ClientConfig::new()
                .set("bootstrap.servers", brokers)
                .set("group.id", group_id)
                .set("enable.partition.eof", "false")
                .set("session.timeout.ms", "6000")
                .set("enable.auto.commit", "false")
                .create()
                .expect("Consumer creation error"),
        );

        let admin_client: AdminClient<_> = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .create()
            .expect("Admin client creation error");

        Self {
            producer,
            consumer,
            admin_client,
        }
    }
}

#[async_trait]
impl MessageBroker for KafkaBroker {
    async fn create_topic(&self, topic_name: &str) -> Result<(), String> {
        let admin_opts = AdminOptions::new();
        let topic = NewTopic::new(topic_name, 1, TopicReplication::Fixed(1));

        match self.admin_client.create_topics(&[topic], &admin_opts).await {
            Ok(results) => match &results[0] {
                Ok(_) => {
                    info!("Topic '{}' created successfully", topic_name);
                    Ok(())
                }
                Err((_, err)) => {
                    error!("Error creating topic '{}': {:?}", topic_name, err);
                    Err(err.to_string())
                }
            },
            Err(e) => {
                error!("Admin operation failed: {:?}", e);
                Err(e.to_string())
            }
        }
    }

    async fn delete_topic(&self, topic: &str) -> Result<(), String> {
        let admin_opts = AdminOptions::new();
        match self.admin_client.delete_topics(&[topic], &admin_opts).await {
            Ok(results) => match &results[0] {
                Ok(_) => {
                    info!("Topic '{}' deleted successfully", topic);
                    Ok(())
                }
                Err((_, err)) => {
                    error!("Error deleting topic '{}': {:?}", topic, err);
                    Err(err.to_string())
                }
            },
            Err(e) => {
                error!("Admin operation failed: {:?}", e);
                Err(e.to_string())
            }
        }
    }

    async fn list_topics(&self) -> Result<Vec<String>, String> {
        match self
            .admin_client
            .inner()
            .fetch_metadata(None, Duration::from_secs(10))
        {
            Ok(metadata) => Ok(metadata
                .topics()
                .iter()
                .map(|t| t.name().to_string())
                .collect()),
            Err(e) => {
                error!("Failed to fetch metadata: {:?}", e);
                Err(e.to_string())
            }
        }
    }

    async fn subscribe<F, Fut>(&self, topic: &str, handler: F) -> Result<(), String>
    where
        F: Fn(PubSubMessage) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send,
    {
        self.consumer
            .subscribe(&[topic])
            .map_err(|e| e.to_string())?;

        let consumer = Arc::clone(&self.consumer);

        tokio::spawn(async move {
            loop {
                match consumer.recv().await {
                    Ok(msg) => {
                        let detached_msg = msg.detach();
                        let payload = detached_msg.payload().map_or(Vec::new(), |p| p.to_vec());
                        let key = detached_msg.key().map(|k| k.to_vec());
                        let message = PubSubMessage { key, payload };

                        handler(message).await;

                        if let Err(e) = consumer.commit_message(&msg, CommitMode::Async) {
                            error!("Failed to commit message: {:?}", e);
                        }
                    }
                    Err(e) => {
                        error!("Error receiving message: {:?}", e);
                    }
                }
            }
        });

        Ok(())
    }

    async fn publish(&self, topic: &str, message: PubSubMessage) -> Result<(), String> {
        let payload = message.payload;
        let key = message.key.unwrap_or_default();

        let record = FutureRecord::to(topic).payload(&payload).key(&key);

        match self.producer.send(record, Duration::from_secs(0)).await {
            Ok(_) => Ok(()),
            Err((e, _)) => {
                error!("Failed to send message: {:?}", e);
                Err(e.to_string())
            }
        }
    }
}
