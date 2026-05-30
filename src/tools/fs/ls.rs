use crate::FS_CONFIG;
use crate::conversation_message::message::{Message, MessageContents, MessageType};
use crate::tools::common::{
    Tool, ToolCallResponse, ToolCallResponseStatus, ToolParameterSchema, ToolParameters,
};
use crate::tools::error::Error;
use crate::tools::fs::common::FsUtils;
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

        if !FS_CONFIG
            .read_whitelist
            .read()
            .await
            .is_match(FsUtils::get_absolute_path(parameters.path.as_str())?)
        {
            Err(Error::WhitelistViolation)?
        }

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
                    r#type: file_type,
                    is_symlink: symlink_dst.is_some(),
                    symlink_dst,
                });
            }
        }

        Ok(Message {
            r#type: MessageType::ToolCallResponse,
            contents: MessageContents::String(to_string(&ToolCallResponse {
                id: id.clone(),
                name: Self::get_name(),
                status: ToolCallResponseStatus::Success,
                content: to_value(list)?,
            })?),
            tool_call_id: Some(id),
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
                            id: id.clone(),
                            name: Self::get_name(),
                            status: ToolCallResponseStatus::Error,
                            content: error.to_string().into(),
                        })
                        .expect("json error"),
                    ),
                    tool_call_id: Some(id),
                })
            }
            Err(error_message) => error_message,
        }
    }

    fn get_name() -> String {
        String::from("fs_ls")
    }

    fn get_descriptions() -> String {
        String::from(
            "## Tool Name: `fs_ls`

### Description
Lists the contents of a specified directory, providing either a basic overview or detailed metadata for each entry found.

### Use Cases
The agent should use this tool when:
-   **Navigation:** Exploring the file system hierarchy to find specific files or subdirectories.
-   **Verification:** Confirming the existence of a file or verifying that a file operation (like creation or deletion) was successful.
-   **Permission Checks:** Investigating Unix permissions or file sizes to troubleshoot access issues or storage constraints.
-   **Symlink Resolution:** Identifying if an entry is a symbolic link and determining its target destination.

### Parameters
-   `path` (string, **required**): The directory path to list. Supports absolute paths, relative paths, and tilde expansion (e.g., `~/Documents`).
-   `detailed` (boolean, **required**):
    -   Set to `false` for a lightweight list (name, type, symlink status).
    -   Set to `true` to include absolute paths, Unix mode permissions, and file size in bytes.

### Behavior
-   **Unix Integration:** This tool uses Unix-specific extensions. Permissions are returned as a numeric mode (bitmask). **Warning: This tool is not cross-platform compatible and is designed for Unix or Unix-like environments; it may fail or behave unexpectedly on non-Unix systems.**
-   **Symlinks:** If a file is a symbolic link, the tool explicitly identifies it and attempts to read the link destination.
-   **Path Resolution:** When listing a relative path, the tool attempts to canonicalize the absolute path for each entry when in `detailed` mode.

### Usage Precautions
-   **UTF-8 Requirement:** The tool will return an error if it encounters path names that are not valid UTF-8.
-   **Empty Directories:** An empty directory will return an empty list `[]` with a `Success` status.
-   **Performance:** Avoid requesting `detailed: true` on directories containing thousands of files unless the metadata is strictly necessary, as canonicalizing every path and fetching metadata incurs additional I/O overhead.
-   **Hidden Files:** The tool lists all entries returned by the filesystem. Unlike a standard shell `ls`, it does not require a special flag to show \"hidden\" files (those starting with a dot), as it iterates through all directory entries provided by the OS.
"
        )
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
                r#type: MessageType::System,
                contents: MessageContents::String("invalid tool parameter format".to_string()),
                tool_call_id: None,
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
