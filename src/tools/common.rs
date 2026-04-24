use crate::api::llm::message::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait Tool {
    type ToolParameters: ToolParameters;
    async fn run(id: String, parameters: Self::ToolParameters) -> Message;
}

pub trait ToolParameters {}

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
