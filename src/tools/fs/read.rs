use crate::api::llm::message::{Message, MessageType};
use crate::tools::common::{Tool, ToolCallResponse, ToolCallResponseStatus, ToolParameters};
use crate::tools::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use tokio::fs::read_to_string;

pub struct Read;

impl Read {
    async fn _run(id: String, mut parameters: ReadParameters) -> Result<Message, Error> {
        let function_name = "fs_read".to_string();

        parameters.path = shellexpand::tilde(parameters.path.as_str()).to_string();

        let output = read_to_string(parameters.path).await?;
        Ok(Message {
            r#type: MessageType::ToolCallResponse,
            string: to_string(&ToolCallResponse {
                id,
                name: function_name,
                status: ToolCallResponseStatus::Success,
                content: output.into(),
            })?,
        })
    }
}

impl Tool for Read {
    type ToolParameters = ReadParameters;
    async fn run(id: String, parameters: Self::ToolParameters) -> Message {
        let function_name = "fs_read".to_string();

        let res = Self::_run(id.clone(), parameters).await;

        res.unwrap_or_else(|error| Message {
            r#type: MessageType::ToolCallResponse,
            string: to_string(&ToolCallResponse {
                id,
                name: function_name,
                status: ToolCallResponseStatus::Error,
                content: error.to_string().into(),
            })
            .expect("json error"),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadParameters {
    pub path: String,
}

impl ToolParameters for ReadParameters {}
