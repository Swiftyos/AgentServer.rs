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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExampleMessage {
    pub id: u32,
    pub content: String,
}

impl ExampleMessage {
    pub fn new(id: u32, content: &str) -> Self {
        Self {
            id,
            content: content.to_string(),
        }
    }
}
