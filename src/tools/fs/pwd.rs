use crate::message::message::{Message, MessageContents, MessageType};
use crate::tools::common::{
    Tool, ToolCallResponse, ToolCallResponseStatus, ToolParameterSchema, ToolParameters,
};
use crate::tools::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string, to_value};
use std::env;

pub struct Pwd;

impl Pwd {
    async fn _run(id: String, parameters: PwdParameters) -> Result<Message, Error> {
        let pwd = env::current_dir()?;

        Ok(Message {
            r#type: MessageType::ToolCallResponse,
            contents: MessageContents::String(to_string(&ToolCallResponse {
                id,
                name: Self::get_name(),
                status: ToolCallResponseStatus::Success,
                content: to_value(pwd)?,
            })?),
        })
    }
}

impl Tool for Pwd {
    async fn run(id: String, parameters: String) -> Message {
        match PwdParameters::from_string(parameters) {
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
        String::from("fs_pwd")
    }

    fn get_descriptions() -> String {
        String::from("Return the current working directory.")
    }

    fn get_parameter_schema() -> ToolParameterSchema {
        ToolParameterSchema {
            r#type: "object".to_string(),
            properties: json!({}),
            required: vec![],
            additional_properties: false,
        }
    }

    fn get_strict() -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PwdParameters;

impl ToolParameters for PwdParameters {
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
