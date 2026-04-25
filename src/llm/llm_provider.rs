use crate::llm::error::Error;
use crate::message::message::Message;
use crate::message::tool_call::ToolKind;

pub trait LLMProvider {
    async fn request(
        &self,
        message_history: Vec<Message>,
        tools: Vec<ToolKind>,
    ) -> Result<Message, Error>;
}
