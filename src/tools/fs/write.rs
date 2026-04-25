use crate::message::message::{Message, MessageContents, MessageType};
use crate::tools::common::{
    Tool, ToolCallResponse, ToolCallResponseStatus, ToolParameterSchema, ToolParameters,
};
use crate::tools::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub struct Write;

impl Write {
    async fn _run(id: String, mut parameters: WriteParameters) -> Result<Message, Error> {
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
            contents: MessageContents::String(to_string(&ToolCallResponse {
                id,
                name: Self::get_name(),
                status: ToolCallResponseStatus::Success,
                content: json!({
                    "bytes_written": parameters.contents.as_bytes().len()
                }),
            })?),
        })
    }
}

impl Tool for Write {
    async fn run(id: String, parameters: String) -> Message {
        match WriteParameters::from_string(parameters) {
            Ok(parameters) => {
                let res = Self::_run(id.clone(), *parameters).await;

                res.unwrap_or_else(|error| Message {
                    r#type: MessageType::ToolCallResponse,
                    contents: MessageContents::String(
                        to_string(&ToolCallResponse {
                            id,
                            name: Self::get_name(),
                            status: ToolCallResponseStatus::Error,
                            content: error.to_string().into(),
                        })
                        .expect("json error"),
                    ),
                })
            }
            Err(error_message) => error_message,
        }
    }

    fn get_name() -> String {
        String::from("fs_write")
    }

    fn get_descriptions() -> String {
        String::from("Write data to the specified file.")
    }

    fn get_parameter_schema() -> ToolParameterSchema {
        ToolParameterSchema {
            r#type: "object".to_string(),
            properties: json!({
                "path": {
                    "type": "string",
                    "description": "The file to write to. Both relative and absolute paths are allowed."
                },
                "mode": {
                    "type": "string",
                    "enum": ["append", "write"],
                    "description": "Append mode: Add <contents> to the end of the file; does not overwrite existing data. Write mode: Erase any existing data and write <contents> to the file."
                },
                "create": {
                    "type": "boolean",
                    "description": "Whether to create the file if it does not exist."
                },
                "contents": {
                    "type": "string",
                    "description": "The data to write."
                }
            }),
            required: vec![
                "path".to_string(),
                "mode".to_string(),
                "create".to_string(),
                "contents".to_string(),
            ],
            additional_properties: false,
        }
    }

    fn get_strict() -> bool {
        true
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

impl ToolParameters for WriteParameters {
    fn from_string(raw_parameters: String) -> Result<Box<Self>, Message> {
        let Ok(parameters) = from_str(raw_parameters.as_str()) else {
            return Err(Message {
                r#type: MessageType::ToolCallResponse,
                contents: MessageContents::String("invalid parameter format".to_string()),
            });
        };
        Ok(parameters)
    }
}
