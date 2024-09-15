# Messaging Library

This library provides a flexible and extensible messaging system for pub/sub
operations, with a focus on Kafka as the underlying message broker.

## Features

- Abstract `MessageBroker` trait for implementing different messaging backends
- Kafka implementation of the `MessageBroker` trait
- Asynchronous API using Tokio
- Serialization support using Serde
- Tracing for logging and diagnostics
- Extensible design for adding new message types and providers

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
messaging = { path = "../libs/messaging" }
```

## Example

```rust
use messaging::kafka::KafkaBroker;
use messaging::pubsub::{MessageBroker, PubSubMessage};

#[tokio::main]
async fn main() -> Result<(), String> {
    let broker = KafkaBroker::new("localhost:9092", "my-group-id").await;

    broker.create_topic("my-topic").await?;

    let message = PubSubMessage {
        key: Some(b"key".to_vec()),
        payload: b"Hello, Kafka!".to_vec(),
    };
    broker.publish("my-topic", message).await?;

    broker.subscribe("my-topic", |msg| async move {
        println!("Received message: {:?}", msg.payload);
    }).await?;

    Ok(())
}
```

## Adding New Message Types

To add a new message type, implement the `Serialize` and `Deserialize` traits for the message type.

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewMessageType {
    field1: String,
    field2: i32,
}
```

## Adding New Message Providers

To add a new message provider, implement the `MessageBroker` trait for the provider. See the `kafka.rs` file for an example.