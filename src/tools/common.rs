use crate::message::message::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait Tool {
    async fn run(id: String, parameters: String) -> Message;
    fn get_name() -> String;
    fn get_descriptions() -> String;
    fn get_parameter_schema() -> ToolParameterSchema;
    fn get_strict() -> bool;
}

pub trait ToolParameters {
    fn from_string(raw_parameters: String) -> Result<Box<Self>, Message>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolParameterSchema {
    pub r#type: String,
    pub properties: Value,
    pub required: Vec<String>,
    pub additional_properties: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCallResponse {
    pub id: String,
    pub name: String,
    pub status: ToolCallResponseStatus,
    pub content: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToolCallResponseStatus {
    Success,
    Error,
}
