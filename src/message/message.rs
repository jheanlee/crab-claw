use crate::message::tool_call::ToolCallRequestMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub r#type: MessageType,
    pub contents: MessageContents,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    System,
    User,
    LLM,
    ToolCallRequest,
    ToolCallResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageContents {
    String(String),
    ToolCallRequests(Vec<ToolCallRequestMessage>),
}

pub trait IntoMessage {
    fn into_message(self) -> Message;
}
