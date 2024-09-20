//! This module demonstrates how to create and implement new message types for the messaging system.
//!
//! # Adding New Message Types
//!
//! To add a new message type:
//! 1. Create a new struct that represents your message.
//! 2. Derive or implement `Serialize` and `Deserialize` traits from serde.
//! 3. Optionally implement methods for creating or manipulating the message.
//! 4. Ensure the new message type can be converted to and from `PubSubMessage`.
//!
//! # Example
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! pub struct NewMessageType {
//!     pub field1: String,
//!     pub field2: i32,
//! }
//!
//! impl NewMessageType {
//!     pub fn new(field1: &str, field2: i32) -> Self {
//!         Self {
//!             field1: field1.to_string(),
//!             field2,
//!         }
//!     }
//! }
//! ```

use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MessageType {
    UserCreated(UserCreatedMessage),
    OrderPlaced(OrderPlacedMessage),
    PaymentProcessed(PaymentProcessedMessage),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserCreatedMessage {
    pub user_id: String,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderPlacedMessage {
    pub order_id: String,
    pub user_id: String,
    pub total_amount: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PaymentProcessedMessage {
    pub payment_id: String,
    pub order_id: String,
    pub status: String,
}

impl MessageType {
    pub fn to_bytes(&self) -> Result<Bytes, bincode::Error> {
        let serialized = bincode::serialize(self)?;
        Ok(Bytes::from(serialized))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization_deserialization() {
        let messages = vec![
            MessageType::UserCreated(UserCreatedMessage {
                user_id: "123".to_string(),
                username: "john_doe".to_string(),
                email: "john@example.com".to_string(),
            }),
            MessageType::OrderPlaced(OrderPlacedMessage {
                order_id: "456".to_string(),
                user_id: "123".to_string(),
                total_amount: 99.99,
            }),
            MessageType::PaymentProcessed(PaymentProcessedMessage {
                payment_id: "789".to_string(),
                order_id: "456".to_string(),
                status: "completed".to_string(),
            }),
        ];

        for original_message in messages {
            let bytes = original_message.to_bytes().unwrap();
            let deserialized_message = MessageType::from_bytes(&bytes).unwrap();
            assert_eq!(original_message, deserialized_message);
        }
    }
}
