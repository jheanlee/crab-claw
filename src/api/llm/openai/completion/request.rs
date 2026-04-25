use crate::api::llm::openai::completion::common::OpenAICompletionMessage;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct OpenAICompletionRequest {
    pub model: String,
    pub messages: Vec<OpenAICompletionMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<OpenAITool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<OpenAIToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAITool {
    pub r#type: String, //  always "function"
    pub function: OpenAIFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAIFunction {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameters: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OpenAIToolChoice {
    Simple(OpenAISimpleToolChoice),
    Tool {
        r#type: String,
        function: OpenAIToolChoiceFunction,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OpenAISimpleToolChoice {
    None,
    Auto,
    Required,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAIToolChoiceFunction {
    pub name: String,
}
