use anyhow::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum BlockDataType {
    String,
    Number,
    Boolean,
    Object(HashMap<String, Box<BlockDataType>>),
    Array(Box<BlockDataType>),
}

impl BlockDataType {
    pub fn validate_input_value(&self, input_value: &BlockValue) -> Result<bool, Error> {
        match &self {
            BlockDataType::String => {
                if !matches!(input_value, BlockValue::String(_)) {
                    return Err(anyhow::anyhow!("Input data does not match input schema"));
                }
            }
            BlockDataType::Number => {
                if !matches!(input_value, BlockValue::Number(_)) {
                    return Err(anyhow::anyhow!("Input data does not match input schema"));
                }
            }
            BlockDataType::Boolean => {
                if !matches!(input_value, BlockValue::Boolean(_)) {
                    return Err(anyhow::anyhow!("Input data does not match input schema"));
                }
            }
            BlockDataType::Object(_) => {
                if !matches!(input_value, BlockValue::Object(_)) {
                    return Err(anyhow::anyhow!("Input data does not match input schema"));
                }

                let data_type_keys: Vec<_> = if let BlockDataType::Object(ref map) = self {
                    map.keys().collect()
                } else {
                    Vec::new()
                };
                let input_value_keys: Vec<_> = if let BlockValue::Object(ref map) = input_value {
                    map.keys().collect()
                } else {
                    Vec::new()
                };

                // Check if input_value_keys and data_type_keys have the same keys
                if !input_value_keys
                    .iter()
                    .all(|key| data_type_keys.contains(key))
                    || !data_type_keys
                        .iter()
                        .all(|key| input_value_keys.contains(key))
                {
                    return Err(anyhow::anyhow!(
                        "Input data keys do not match input schema keys"
                    ));
                }

                // now for each key we need to recursively validate the input_value
                for key in input_value_keys {
                    let input_value = if let BlockValue::Object(ref map) = input_value {
                        map.get(key).unwrap()
                    } else {
                        unreachable!()
                    };
                    let data_type = if let BlockDataType::Object(ref map) = self {
                        map.get(key).unwrap()
                    } else {
                        unreachable!()
                    };
                    data_type.validate_input_value(input_value)?;
                }
                return Ok(true);
            }
            BlockDataType::Array(_) => {
                if !matches!(input_value, BlockValue::Array(_)) {
                    return Err(anyhow::anyhow!("Input data does not match input schema"));
                }
                let data_type = if let BlockDataType::Array(ref data_type) = self {
                    data_type
                } else {
                    unreachable!()
                };
                let input_value = if let BlockValue::Array(ref vec) = input_value {
                    vec
                } else {
                    unreachable!()
                };
                for value in input_value {
                    data_type.validate_input_value(value)?;
                }
                return Ok(true);
            }
        }
        Ok(true)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BlockIOSchema {
    pub name: String,
    pub description: String,
    pub data_type: BlockDataType,
    pub default_value: Option<BlockValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BlockConfig {
    pub is_secret: bool,
    pub name: String,
    pub value: BlockDataType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum BlockValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Object(HashMap<String, BlockValue>),
    Array(Vec<BlockValue>),
}

pub struct InputData {
    pub name: String,
    pub value: BlockValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AgentBlock {
    pub id: Uuid,
    pub name: String,
    pub input_schema: HashMap<String, BlockIOSchema>,
    pub output_schema: HashMap<String, BlockIOSchema>,
    pub config: HashMap<String, BlockConfig>,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl AgentBlock {
    pub fn validate_input_data(
        &self,
        input_data: HashMap<String, InputData>,
    ) -> Result<bool, Error> {
        for (name, input) in self.input_schema.clone() {
            if !input_data.contains_key(&name) && input.default_value.is_none() {
                return Err(anyhow::anyhow!("Input data does not match input schema"));
            }

            let input_value = input_data.get(&name).unwrap();
            let input_schema = input.data_type;
            input_schema.validate_input_value(&input_value.value)?;
        }
        Ok(true)
    }

    pub fn run(&self, input: &str) -> Result<String, Error> {
        Ok(input.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_validate_input_data() {
        let block = AgentBlock {
            id: Uuid::new_v4(),
            name: "Test Block".to_string(),
            input_schema: {
                let mut schema = HashMap::new();
                schema.insert(
                    "input1".to_string(),
                    BlockIOSchema {
                        name: "input1".to_string(),
                        description: "".to_string(),
                        data_type: BlockDataType::String,
                        default_value: None,
                    },
                );
                schema.insert(
                    "input2".to_string(),
                    BlockIOSchema {
                        name: "input2".to_string(),
                        description: "".to_string(),
                        data_type: BlockDataType::Number,
                        default_value: Some(BlockValue::Number(5.0)),
                    },
                );
                schema
            },
            output_schema: HashMap::new(),
            config: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Test case 1: Valid input data
        let mut input_data = HashMap::new();
        input_data.insert(
            "input1".to_string(),
            InputData {
                name: "input1".to_string(),
                value: BlockValue::String("test".to_string()),
            },
        );
        input_data.insert(
            "input2".to_string(),
            InputData {
                name: "input2".to_string(),
                value: BlockValue::Number(10.0),
            },
        );

        assert!(block.validate_input_data(input_data).is_ok());

        // Test case 2: Missing required input
        let mut input_data = HashMap::new();
        input_data.insert(
            "input2".to_string(),
            InputData {
                name: "input2".to_string(),
                value: BlockValue::Number(10.0),
            },
        );

        assert!(block.validate_input_data(input_data).is_err());

        // Test case 3: Mismatched input type
        let mut input_data = HashMap::new();
        input_data.insert(
            "input1".to_string(),
            InputData {
                name: "input1".to_string(),
                value: BlockValue::Number(5.0), // Should be String
            },
        );
        input_data.insert(
            "input2".to_string(),
            InputData {
                name: "input2".to_string(),
                value: BlockValue::Number(10.0),
            },
        );

        assert!(block.validate_input_data(input_data).is_err());
    }
}
