use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub r#type: MessageType,
    pub string: String,
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
