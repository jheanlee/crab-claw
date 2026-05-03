use crate::conversation_message::tool_call::ToolCallRequestMessage;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

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

impl Into<String> for MessageContents {
    fn into(self) -> String {
        match self {
            MessageContents::String(string) => string,
            MessageContents::ToolCallRequests(requests) => {
                to_string(&requests).expect("serialisation failed")
            }
        }
    }
}

pub trait IntoMessage {
    fn into_message(self) -> Message;
}
