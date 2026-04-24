use crate::api::llm::message::{Message, MessageType};
use crate::tools::common::{Tool, ToolCallResponse, ToolCallResponseStatus, ToolParameters};
use crate::tools::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{to_string, to_value};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use tokio::fs::{read_dir, read_link};

pub struct Ls;

impl Ls {
    async fn _run(id: String, mut parameters: LsParameters) -> Result<Message, Error> {
        let function_name = "fs_ls".to_string();

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
            string: to_string(&ToolCallResponse {
                id,
                name: function_name,
                status: ToolCallResponseStatus::Success,
                content: to_value(list)?,
            })?,
        })
    }
}

impl Tool for Ls {
    type ToolParameters = LsParameters;

    async fn run(id: String, parameters: Self::ToolParameters) -> Message {
        let function_name = "fs_ls".to_string();

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
pub struct LsParameters {
    pub path: String,
    pub detailed: bool,
}

impl ToolParameters for LsParameters {}

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
