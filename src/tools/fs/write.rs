use crate::api::llm::message::{Message, MessageType};
use crate::tools::common::{Tool, ToolCallResponse, ToolCallResponseStatus, ToolParameters};
use crate::tools::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub struct Write;

impl Write {
    async fn _run(id: String, mut parameters: WriteParameters) -> Result<Message, Error> {
        let function_name = "fs_write".to_string();

        parameters.path = shellexpand::tilde(parameters.path.as_str()).to_string();

        let mut open_options = OpenOptions::new();
        open_options.create(parameters.create);

        match parameters.mode {
            WriteMode::Append => {
                open_options.append(true);
            }
            WriteMode::Write => {
                open_options.write(true).truncate(true);
            }
        }

        let mut file = open_options.open(parameters.path).await?;
        file.write_all(parameters.contents.as_bytes()).await?;
        file.flush().await?;

        Ok(Message {
            r#type: MessageType::ToolCallResponse,
            string: to_string(&ToolCallResponse {
                id,
                name: function_name,
                status: ToolCallResponseStatus::Success,
                content: json!({
                    "bytes_written": parameters.contents.as_bytes().len()
                }),
            })?,
        })
    }
}

impl Tool for Write {
    type ToolParameters = WriteParameters;
    async fn run(id: String, parameters: Self::ToolParameters) -> Message {
        let function_name = "fs_write".to_string();

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
pub struct WriteParameters {
    pub path: String,
    pub mode: WriteMode,
    pub create: bool,
    pub contents: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WriteMode {
    Append,
    Write,
}

impl ToolParameters for WriteParameters {}
