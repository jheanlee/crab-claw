use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OpenAICompletionMessageRole {
    System,
    User,
    Assistant,
    Tool,
    Function,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAICompletionMessage {
    pub role: OpenAICompletionMessageRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<OpenAICompletionToolCall>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAICompletionToolCall {
    pub id: String,
    pub r#type: String, // Usually "function"
    pub function: OpenAICompletionToolFunctionCall,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAICompletionToolFunctionCall {
    pub name: String,
    pub arguments: Value,
}
