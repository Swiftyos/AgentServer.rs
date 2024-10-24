// use anyhow::Error;
// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use sqlx::FromRow;
// use std::collections::HashMap;
// use uuid::Uuid;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type", content = "value")]
// pub enum BlockDataType {
//     String,
//     Number,
//     Boolean,
//     Object(HashMap<String, Box<BlockDataType>>),
//     Array(Box<BlockDataType>),
// }

// impl BlockDataType {
//     pub fn validate_input_value(&self, input_value: &BlockValue) -> Result<bool, Error> {
//         match &self {
//             BlockDataType::String => {
//                 if !matches!(input_value, BlockValue::String(_)) {
//                     return Err(anyhow::anyhow!("Input data does not match input schema"));
//                 }
//             }
//             BlockDataType::Number => {
//                 if !matches!(input_value, BlockValue::Number(_)) {
//                     return Err(anyhow::anyhow!("Input data does not match input schema"));
//                 }
//             }
//             BlockDataType::Boolean => {
//                 if !matches!(input_value, BlockValue::Boolean(_)) {
//                     return Err(anyhow::anyhow!("Input data does not match input schema"));
//                 }
//             }
//             BlockDataType::Object(_) => {
//                 if !matches!(input_value, BlockValue::Object(_)) {
//                     return Err(anyhow::anyhow!("Input data does not match input schema"));
//                 }

//                 let data_type_keys: Vec<_> = if let BlockDataType::Object(ref map) = self {
//                     map.keys().collect()
//                 } else {
//                     Vec::new()
//                 };
//                 let input_value_keys: Vec<_> = if let BlockValue::Object(ref map) = input_value {
//                     map.keys().collect()
//                 } else {
//                     Vec::new()
//                 };

//                 // Check if input_value_keys and data_type_keys have the same keys
//                 if !input_value_keys
//                     .iter()
//                     .all(|key| data_type_keys.contains(key))
//                     || !data_type_keys
//                         .iter()
//                         .all(|key| input_value_keys.contains(key))
//                 {
//                     return Err(anyhow::anyhow!(
//                         "Input data keys do not match input schema keys"
//                     ));
//                 }

//                 // now for each key we need to recursively validate the input_value
//                 for key in input_value_keys {
//                     let input_value = if let BlockValue::Object(ref map) = input_value {
//                         map.get(key).unwrap()
//                     } else {
//                         unreachable!()
//                     };
//                     let data_type = if let BlockDataType::Object(ref map) = self {
//                         map.get(key).unwrap()
//                     } else {
//                         unreachable!()
//                     };
//                     data_type.validate_input_value(input_value)?;
//                 }
//                 return Ok(true);
//             }
//             BlockDataType::Array(_) => {
//                 if !matches!(input_value, BlockValue::Array(_)) {
//                     return Err(anyhow::anyhow!("Input data does not match input schema"));
//                 }
//                 let data_type = if let BlockDataType::Array(ref data_type) = self {
//                     data_type
//                 } else {
//                     unreachable!()
//                 };
//                 let input_value = if let BlockValue::Array(ref vec) = input_value {
//                     vec
//                 } else {
//                     unreachable!()
//                 };
//                 for value in input_value {
//                     data_type.validate_input_value(value)?;
//                 }
//                 return Ok(true);
//             }
//         }
//         Ok(true)
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct BlockIOSchema {
//     pub name: String,
//     pub description: String,
//     pub data_type: BlockDataType,
//     pub default_value: Option<BlockValue>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct BlockConfig {
//     pub is_secret: bool,
//     pub schema: BlockIOSchema,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type", content = "value")]
// pub enum BlockValue {
//     String(String),
//     Number(f64),
//     Boolean(bool),
//     Object(HashMap<String, BlockValue>),
//     Array(Vec<BlockValue>),
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct IOData {
//     pub name: String,
//     pub value: BlockValue,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct BlockDetails {
//     pub id: Uuid,
//     pub name: String,
//     pub input_schema: HashMap<String, BlockIOSchema>,
//     pub output_schema: HashMap<String, BlockIOSchema>,
//     pub config: HashMap<String, BlockConfig>,
//     #[serde(with = "chrono::serde::ts_seconds")]
//     pub created_at: DateTime<Utc>,
//     #[serde(with = "chrono::serde::ts_seconds")]
//     pub updated_at: DateTime<Utc>,
// }

// trait AgentBlock {
//     fn block_details(&self) -> BlockDetails;
//     fn get_input_value_or_default(
//         &self,
//         input_data: HashMap<String, IOData>,
//         name: &str,
//     ) -> Result<IOData, Error> {
//         let input_data: Result<IOData, Error> = match input_data.get(name) {
//             Some(value) => Ok(value.clone()),
//             None => match self.block_details().input_schema.get(name) {
//                 Some(schema) => match &schema.default_value {
//                     Some(default_value) => Ok(IOData {
//                         name: name.to_string(),
//                         value: default_value.clone(),
//                     }),
//                     None => Err(anyhow::anyhow!(
//                         "Input data does not match input schema and no default value is provided"
//                     )),
//                 },
//                 None => Err(anyhow::anyhow!(
//                     "Input schema does not contain the specified name"
//                 )),
//             },
//         };
//         input_data
//     }

//     fn get_config_value_or_default(
//         &self,
//         config_data: HashMap<String, IOData>,
//         name: &str,
//     ) -> Result<IOData, Error> {
//         let config_data: Result<IOData, Error> = match config_data.get(name) {
//             Some(value) => Ok(value.clone()),
//             None => match self.block_details().config.get(name) {
//                 Some(cfg) => match &cfg.schema.default_value {
//                     Some(default_value) => Ok(IOData {
//                         name: name.to_string(),
//                         value: default_value.clone(),
//                     }),
//                     None => Err(anyhow::anyhow!(
//                         "Input data does not match input schema and no default value is provided"
//                     )),
//                 },
//                 None => Err(anyhow::anyhow!(
//                     "Input schema does not contain the specified name"
//                 )),
//             },
//         };
//         config_data
//     }

//     fn run(
//         &self,
//         input_data: HashMap<String, IOData>,
//         config_data: HashMap<String, IOData>,
//     ) -> Result<impl Iterator<Item = IOData>, Error>;

//     fn validate_input_data(
//         &self,
//         input_data: HashMap<String, IOData>,
//         config_data: HashMap<String, IOData>,
//     ) -> Result<bool, Error> {
//         for (name, input) in &self.block_details().input_schema {
//             if !input_data.contains_key(name) && input.default_value.is_none() {
//                 return Err(anyhow::anyhow!("Input data does not match input schema"));
//             }

//             let input_value = input_data.get(name).unwrap();
//             let input_schema = &input.data_type;
//             input_schema.validate_input_value(&input_value.value)?;
//         }
//         for (name, config) in &self.block_details().config {
//             if !config_data.contains_key(name) && config.schema.default_value.is_none() {
//                 return Err(anyhow::anyhow!("Config data does not match config schema"));
//             }

//             let config_value = config_data.get(name).unwrap();
//             let config_schema = &config.schema.data_type;
//             config_schema.validate_input_value(&config_value.value)?;
//         }
//         Ok(true)
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PrintBlock {}

// impl AgentBlock for PrintBlock {
//     fn block_details(&self) -> BlockDetails {
//         let block_details = BlockDetails {
//             id: Uuid::new_v4(),
//             name: "Print Block".to_string(),
//             input_schema: {
//                 let mut schema = HashMap::new();
//                 schema.insert(
//                     "value".to_string(),
//                     BlockIOSchema {
//                         name: "value".to_string(),
//                         description: "Input value for the Print Block".to_string(),
//                         data_type: BlockDataType::String,
//                         default_value: None,
//                     },
//                 );
//                 schema
//             },
//             output_schema: {
//                 let mut schema = HashMap::new();
//                 schema.insert(
//                     "output".to_string(),
//                     BlockIOSchema {
//                         name: "output".to_string(),
//                         description: "Output value for the Print Block".to_string(),
//                         data_type: BlockDataType::String,
//                         default_value: None,
//                     },
//                 );
//                 schema
//             },
//             config: {
//                 let mut config = HashMap::new();
//                 config.insert(
//                     "capitalise".to_string(),
//                     BlockConfig {
//                         is_secret: false,
//                         schema: BlockIOSchema {
//                             name: "capitalise".to_string(),
//                             description: "Whether to capitalise the output".to_string(),
//                             data_type: BlockDataType::Boolean,
//                             default_value: Some(BlockValue::Boolean(false)),
//                         },
//                     },
//                 );
//                 config
//             },
//             created_at: Utc::now(),
//             updated_at: Utc::now(),
//         };
//         block_details
//     }

//     fn run(
//         &self,
//         input_data: HashMap<String, IOData>,
//         config_data: HashMap<String, IOData>,
//     ) -> Result<impl Iterator<Item = IOData>, Error> {
//         let value = input_data.get("value").unwrap();
//         let value_str = match &value.value {
//             BlockValue::String(s) => s,
//             _ => return Err(anyhow::anyhow!("Invalid input type")),
//         };
//         let capitalise = match self
//             .get_config_value_or_default(config_data, "capitalise")?
//             .value
//         {
//             BlockValue::Boolean(b) => b,
//             _ => return Err(anyhow::anyhow!("Invalid config type for 'capitalise'")),
//         };

//         // Create an iterator that yields the output value
//         Ok(std::iter::once(if capitalise {
//             IOData {
//                 name: "output".to_string(),
//                 value: BlockValue::String(value_str.to_uppercase()),
//             }
//         } else {
//             IOData {
//                 name: "output".to_string(),
//                 value: BlockValue::String(value_str.to_string()),
//             }
//         }))
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use chrono::Utc;

//     #[test]
//     fn test_validate_input_data() {
//         let block = PrintBlock {};

//         // Test case 1: Valid input data
//         let mut input_data = HashMap::new();
//         input_data.insert(
//             "input1".to_string(),
//             IOData {
//                 name: "input1".to_string(),
//                 value: BlockValue::String("test".to_string()),
//             },
//         );
//         input_data.insert(
//             "input2".to_string(),
//             IOData {
//                 name: "input2".to_string(),
//                 value: BlockValue::Number(10.0),
//             },
//         );
//         let mut config_data = HashMap::new();
//         config_data.insert(
//             "capitalise".to_string(),
//             IOData {
//                 name: "capitalise".to_string(),
//                 value: BlockValue::Boolean(true),
//             },
//         );

//         assert!(block.validate_input_data(input_data, config_data).is_ok());

//         // Test case 2: Missing required input
//         let mut input_data = HashMap::new();
//         input_data.insert(
//             "input2".to_string(),
//             IOData {
//                 name: "input2".to_string(),
//                 value: BlockValue::Number(10.0),
//             },
//         );

//         let mut config_data = HashMap::new();
//         config_data.insert(
//             "capitalise".to_string(),
//             IOData {
//                 name: "capitalise".to_string(),
//                 value: BlockValue::Boolean(true),
//             },
//         );

//         assert!(block.validate_input_data(input_data, config_data).is_err());

//         // Test case 3: Mismatched input type
//         let mut input_data = HashMap::new();
//         input_data.insert(
//             "input1".to_string(),
//             IOData {
//                 name: "input1".to_string(),
//                 value: BlockValue::Number(5.0), // Should be String
//             },
//         );
//         input_data.insert(
//             "input2".to_string(),
//             IOData {
//                 name: "input2".to_string(),
//                 value: BlockValue::Number(10.0),
//             },
//         );

//         let mut config_data = HashMap::new();
//         config_data.insert(
//             "capitalise".to_string(),
//             IOData {
//                 name: "capitalise".to_string(),
//                 value: BlockValue::Boolean(true),
//             },
//         );

//         assert!(block.validate_input_data(input_data, config_data).is_err());
//     }
// }
