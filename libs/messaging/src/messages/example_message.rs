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
