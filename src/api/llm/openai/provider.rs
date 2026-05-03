use crate::api::llm::openai::completion::common::{
    OpenAICompletionMessage, OpenAICompletionMessageRole,
};
use crate::api::llm::openai::completion::request::OpenAIToolChoice::Simple;
use crate::api::llm::openai::completion::request::{
    OpenAICompletionRequest, OpenAISimpleToolChoice,
};
use crate::api::llm::openai::completion::response::OpenAICompletionResponse;
use crate::conversation_message::message::{IntoMessage, Message};
use crate::conversation_message::tool_call::ToolKind;
use crate::llm::llm_provider::LLMProvider;
use async_trait::async_trait;
use serde_json::to_string;

pub struct OpenAIProvider {
    pub model: String,
    pub api_base: String,
    pub api_key: String,
}

impl OpenAIProvider {
    pub fn new(model: String, api_base: String) -> Self {
        OpenAIProvider {
            model,
            api_base,
            api_key: "".to_string(),
        }
    }
}

#[async_trait]
impl LLMProvider for OpenAIProvider {
    async fn request(
        &self,
        message_history: Vec<Message>,
        tools: Vec<ToolKind>,
    ) -> Result<Message, crate::llm::error::Error> {
        let request = OpenAICompletionRequest {
            model: self.model.clone(),
            messages: message_history
                .iter()
                .map(|message| OpenAICompletionMessage::from(message.clone()))
                .collect(),
            n: Some(1),
            max_completion_tokens: None,
            max_tokens: None,
            tools: Some(tools.iter().map(|tool| tool.clone().into()).collect()),
            tool_choice: Some(Simple(OpenAISimpleToolChoice::Auto)),
            parallel_tool_calls: None,
            temperature: None,
        };

        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:4000/v1/chat/completions")
            .body(to_string(&request).expect("serialisation failed"))
            .send()
            .await?;

        let res_bytes = res.bytes().await?;
        match serde_json::from_slice::<OpenAICompletionResponse>(&res_bytes) {
            Ok(response) => {
                if response.choices.len() != 1 {
                    Err(crate::llm::error::Error::InvalidChoiceCount)?
                }
                Ok(OpenAICompletionMessage {
                    role: OpenAICompletionMessageRole::Assistant,
                    content: response.choices[0].message.content.clone(),
                    tool_calls: response.choices[0].message.tool_calls.clone(),
                }
                .into_message())
            }
            Err(_) => Err(crate::llm::error::Error::LLMError(
                String::from_utf8_lossy(res_bytes.as_ref()).to_string(),
            )),
        }
    }
}
