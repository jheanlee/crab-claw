use crate::message::message::{Message, MessageContents, MessageType};
use crate::tools::common::{
    Tool, ToolCallResponse, ToolCallResponseStatus, ToolParameterSchema, ToolParameters,
};
use crate::tools::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string, to_value};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use tokio::fs::{read_dir, read_link};

pub struct Ls;

impl Ls {
    pub fn new() -> Self {
        Ls {}
    }

    async fn _run(id: String, mut parameters: LsParameters) -> Result<Message, Error> {
        parameters.path = shellexpand::tilde(parameters.path.as_str()).to_string();

        let mut reader = read_dir(parameters.path.clone()).await?;
        let mut list = Vec::new();

        while let Some(entry) = reader.next_entry().await? {
            let path = entry.path();
            let absolute_path = if path.is_absolute() {
                path.clone()
            } else {
                Path::new(&parameters.path)
                    .join(entry.file_name())
                    .canonicalize()?
            };

            let metadata = entry.metadata().await?;
            let permissions = metadata.permissions().mode();
            let file_size = metadata.size();
            let file_type;
            if metadata.is_dir() {
                file_type = "directory".to_string();
            } else {
                file_type = "file".to_string();
            }

            let mut symlink_dst = None;
            if metadata.is_symlink() {
                symlink_dst = Some(
                    read_link(absolute_path.clone())
                        .await?
                        .to_str()
                        .ok_or(Error::NonUTF8PathName)?
                        .to_string(),
                );
            }

            if parameters.detailed {
                list.push(LsFileEntry::Detailed {
                    path: path.to_str().ok_or(Error::NonUTF8PathName)?.to_string(),
                    absolute_path: absolute_path
                        .to_str()
                        .ok_or(Error::NonUTF8PathName)?
                        .to_string(),
                    r#type: file_type,
                    is_symlink: symlink_dst.is_some(),
                    symlink_dst,
                    permissions,
                    size: file_size,
                });
            } else {
                list.push(LsFileEntry::Simple {
                    path: path.to_str().ok_or(Error::NonUTF8PathName)?.to_string(),
                    absolute_path: absolute_path
                        .to_str()
                        .ok_or(Error::NonUTF8PathName)?
                        .to_string(),
                    r#type: file_type,
                    is_symlink: symlink_dst.is_some(),
                    symlink_dst,
                });
            }
        }

        Ok(Message {
            r#type: MessageType::ToolCallResponse,
            contents: MessageContents::String(to_string(&ToolCallResponse {
                id,
                name: Self::get_name(),
                status: ToolCallResponseStatus::Success,
                content: to_value(list)?,
            })?),
        })
    }
}

impl Tool for Ls {
    async fn run(id: String, parameters: String) -> Message {
        match LsParameters::from_string(parameters) {
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
        String::from("fs_ls")
    }

    fn get_descriptions() -> String {
        String::from("List contents of the specified directory.")
    }

    fn get_parameter_schema() -> ToolParameterSchema {
        ToolParameterSchema {
            r#type: "object".to_string(),
            properties: json!({
                "path": {
                    "type": "string",
                    "description": "The directory path to list. Both relative and absolute paths are allowed."
                },
                "detailed": {
                    "type": "boolean",
                    "description": "Whether to include metadata like file size and unix permissions."
                }
            }),
            required: vec!["path".to_string(), "detailed".to_string()],
            additional_properties: false,
        }
    }

    fn get_strict() -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LsParameters {
    pub path: String,
    pub detailed: bool,
}

impl ToolParameters for LsParameters {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum LsFileEntry {
    Simple {
        path: String,
        absolute_path: String,
        r#type: String,
        is_symlink: bool,
        symlink_dst: Option<String>,
    },
    Detailed {
        path: String,
        absolute_path: String,
        r#type: String,
        is_symlink: bool,
        symlink_dst: Option<String>,
        permissions: u32,
        size: u64,
    },
}
