use crate::llm::error::Error;
use crate::message::message::Message;
use crate::message::tool_call::ToolKind;
use async_trait::async_trait;

#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn request(
        &self,
        message_history: Vec<Message>,
        tools: Vec<ToolKind>,
    ) -> Result<Message, Error>;
}
