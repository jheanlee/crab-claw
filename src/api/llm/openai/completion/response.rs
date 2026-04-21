use crate::api::llm::openai::completion::common::OpenAICompletionMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAICompletionResponse {
    pub id: String,
    pub model: String,
    pub object: String, //  "chat.completion"
    pub created: u64,   //  timestamp
    pub choices: Vec<OpenAICompletionResponseChoice>,
    pub usage: OpenAICompletionResponseUsage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAICompletionResponseChoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<OpenAICompletionResponseChoiceFinishReason>,
    pub index: u32,
    pub message: OpenAICompletionMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OpenAICompletionResponseChoiceFinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAICompletionResponseUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
