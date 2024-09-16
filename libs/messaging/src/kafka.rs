//! Kafka implementation of the MessageBroker trait.
//!
//! This module provides a Kafka-based implementation of the `MessageBroker` trait,
//! allowing for pub/sub operations using Apache Kafka as the underlying message broker.
//!
//! The `KafkaBroker` struct encapsulates the necessary Kafka components (producer, consumer, and admin client)
//! and implements the `MessageBroker` trait to provide a consistent interface for messaging operations.
//!
//! # Example
//!
//! ```rust
//! use messaging::kafka::KafkaBroker;
//! use messaging::pubsub::{MessageBroker, PubSubMessage};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let broker = KafkaBroker::new("localhost:9092", "my-group-id").await;
//!
//!     broker.create_topic("my-topic").await?;
//!
//!     let message = PubSubMessage {
//!         key: Some(b"key".to_vec()),
//!         payload: b"Hello, Kafka!".to_vec(),
//!     };
//!     broker.publish("my-topic", message).await?;
//!
//!     broker.subscribe("my-topic", |msg| async move {
//!         println!("Received message: {:?}", msg.payload);
//!     }).await?;
//!
//!     Ok(())
//! }
//! ```

use crate::pubsub::{MessageBroker, PubSubMessage};
use anyhow::Result;
use async_trait::async_trait;
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

/// A struct representing a Kafka-based message broker.
pub struct KafkaBroker {
    producer: Arc<FutureProducer>,
    consumer: Arc<StreamConsumer>,
    admin_client: AdminClient<rdkafka::client::DefaultClientContext>,
}

impl KafkaBroker {
    /// Creates a new `KafkaBroker` instance.
    ///
    /// # Arguments
    ///
    /// * `brokers` - A comma-separated list of host and port pairs that are the addresses of the Kafka brokers in a "bootstrap" Kafka cluster.
    /// * `group_id` - The name of the consumer group this consumer belongs to.
    ///
    /// # Returns
    ///
    /// A new `KafkaBroker` instance.
    pub async fn new(brokers: &str, group_id: &str) -> Self {
        let producer = Arc::new(
            ClientConfig::new()
                .set("bootstrap.servers", brokers)
                .create()
                .expect("Producer creation error"),
        );

        let consumer = Arc::new(
            ClientConfig::new()
                .set("bootstrap.servers", brokers)
                .set("group.id", group_id)
                .set("enable.partition.eof", "false")
                .set("session.timeout.ms", "6000")
                .set("enable.auto.commit", "false")
                .set("auto.offset.reset", "earliest")
                .create()
                .expect("Consumer creation error"),
        );

        let admin_client = ClientConfig::new()
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
    async fn create_topic(&self, topic_name: &str) -> Result<()> {
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
                    Err(anyhow::anyhow!("Failed to create topic: {}", err))
                }
            },
            Err(e) => {
                error!("Admin operation failed: {:?}", e);
                Err(anyhow::anyhow!("Admin operation failed: {}", e))
            }
        }
    }

    async fn delete_topic(&self, topic: &str) -> Result<()> {
        let admin_opts = AdminOptions::new();
        match self.admin_client.delete_topics(&[topic], &admin_opts).await {
            Ok(results) => match &results[0] {
                Ok(_) => {
                    info!("Topic '{}' deleted successfully", topic);
                    Ok(())
                }
                Err((_, err)) => {
                    error!("Error deleting topic '{}': {:?}", topic, err);
                    Err(anyhow::anyhow!("Failed to delete topic: {}", err))
                }
            },
            Err(e) => {
                error!("Admin operation failed: {:?}", e);
                Err(anyhow::anyhow!("Admin operation failed: {}", e))
            }
        }
    }

    async fn list_topics(&self) -> Result<Vec<String>> {
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
                Err(anyhow::anyhow!("Failed to fetch metadata: {}", e))
            }
        }
    }

    async fn subscribe<F, Fut>(&self, topic: &str, handler: F) -> Result<()>
    where
        F: Fn(PubSubMessage) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send,
    {
        self.consumer
            .subscribe(&[topic])
            .map_err(|e| anyhow::anyhow!("Failed to subscribe: {}", e))?;

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

    async fn publish(&self, topic: &str, message: PubSubMessage) -> Result<()> {
        let payload = message.payload;
        let key = message.key.unwrap_or_default();

        let record = FutureRecord::to(topic).payload(&payload).key(&key);

        match self.producer.send(record, Duration::from_secs(0)).await {
            Ok(_) => Ok(()),
            Err((e, _)) => {
                error!("Failed to send message: {:?}", e);
                Err(anyhow::anyhow!("Failed to send message: {}", e))
            }
        }
    }
}
