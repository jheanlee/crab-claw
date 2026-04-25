use crate::message::message::{Message, MessageContents, MessageType};
use crate::tools::common::{
    Tool, ToolCallResponse, ToolCallResponseStatus, ToolParameterSchema, ToolParameters,
};
use crate::tools::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string, to_value};
use tokio::fs::read_to_string;

pub struct Read;

impl Read {
    async fn _run(id: String, mut parameters: ReadParameters) -> Result<Message, Error> {
        parameters.path = shellexpand::tilde(parameters.path.as_str()).to_string();

        let output = read_to_string(parameters.path).await?;
        Ok(Message {
            r#type: MessageType::ToolCallResponse,
            contents: MessageContents::String(to_string(&ToolCallResponse {
                id,
                name: Self::get_name(),
                status: ToolCallResponseStatus::Success,
                content: to_value(output)?,
            })?),
        })
    }
}

impl Tool for Read {
    async fn run(id: String, parameters: String) -> Message {
        match ReadParameters::from_string(parameters) {
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
        String::from("fs_read")
    }

    fn get_descriptions() -> String {
        String::from(
            "Read all content of the specified file. The output is in the UTF-8 encoding. Note: May cause the agent to use too much tokens if file size is too large; use with caution.",
        )
    }

    fn get_parameter_schema() -> ToolParameterSchema {
        ToolParameterSchema {
            r#type: "object".to_string(),
            properties: json!({
                "path": {
                    "type": "string",
                    "description": "The file to read. Both relative and absolute paths are allowed."
                }
            }),
            required: vec!["path".to_string()],
            additional_properties: false,
        }
    }

    fn get_strict() -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadParameters {
    pub path: String,
}

impl ToolParameters for ReadParameters {
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
