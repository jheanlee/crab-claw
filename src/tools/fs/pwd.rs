use crate::api::llm::message::{Message, MessageType};
use crate::tools::common::{Tool, ToolCallResponse, ToolCallResponseStatus, ToolParameters};
use crate::tools::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{to_string, to_value};
use std::env;

pub struct Pwd;

impl Pwd {
    async fn _run(id: String, parameters: PwdParameters) -> Result<Message, Error> {
        let function_name = "fs_pwd".to_string();

        let pwd = env::current_dir()?;

        Ok(Message {
            r#type: MessageType::ToolCallResponse,
            string: to_string(&ToolCallResponse {
                id,
                name: function_name,
                status: ToolCallResponseStatus::Success,
                content: to_value(pwd)?,
            })?,
        })
    }
}

impl Tool for Pwd {
    type ToolParameters = PwdParameters;

    async fn run(id: String, parameters: Self::ToolParameters) -> Message {
        let function_name = "fs_pwd".to_string();

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
pub struct PwdParameters;

impl ToolParameters for PwdParameters {}
