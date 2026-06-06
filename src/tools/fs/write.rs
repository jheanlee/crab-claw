/*
 * Copyright 2026 Jhe-An Lee
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::FS_CONFIG;
use crate::conversation_message::message::{Message, MessageContents, MessageType};
use crate::tools::common::{
    Tool, ToolCallResponse, ToolCallResponseStatus, ToolParameterSchema, ToolParameters,
};
use crate::tools::error::Error;
use crate::tools::fs::common::FsUtils;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub struct Write;

impl Write {
    pub fn new() -> Self {
        Write {}
    }

    async fn _run(id: String, mut parameters: WriteParameters) -> Result<Message, Error> {
        parameters.path = shellexpand::tilde(parameters.path.as_str()).to_string();

        if !FS_CONFIG
            .write_whitelist
            .read()
            .await
            .is_match(FsUtils::get_absolute_path(parameters.path.as_str())?)
        {
            Err(Error::WhitelistViolation)?
        }

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
                id: id.clone(),
                name: Self::get_name(),
                status: ToolCallResponseStatus::Success,
                content: json!({
                    "bytes_written": parameters.contents.as_bytes().len()
                }),
            })?),
            tool_call_id: Some(id),
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
        String::from("fs_write")
    }

    fn get_descriptions() -> String {
        String::from(
            "## Tool Name: `fs_write`

### Description
Writes text data to a specified file, with support for creating new files, overwriting existing ones, or appending to them.

### Use Cases
The agent should use this tool when:
-   **File Creation:** Generating new source files, configuration files, or documentation.
-   **Logging:** Appending status updates or audit trail entries to an existing log file.
-   **Code Modification:** Overwriting a file with updated logic or refactored code.
-   **Persistent Storage:** Saving session data or results for later use by the user or the agent.

### Parameters
-   `path` (string, **required**): The destination file path. Supports absolute paths, relative paths, and tilde expansion (e.g., `~/scripts/test.sh`).
-   `mode` (string, **required**):
    -   `\"write\"`: Erases all existing content in the file and writes the new `contents`.
    -   `\"append\"`: Preserves existing data and adds the new `contents` to the end of the file.
-   `create` (boolean, **required**):
    -   Set to `true` to create the file if it does not already exist.
    -   Set to `false` to ensure writing only occurs if the file is already present (fails if missing).
-   `contents` (string, **required**): The string data to write to the file.

### Behavior
-   **Tilde Expansion:** Automatically resolves `~` to the user's home directory.
-   **Asynchronous Processing:** Uses `tokio::fs` to perform non-blocking file I/O, flushing buffers immediately after writing to ensure data integrity.
-   **Output:** Returns the total number of bytes successfully written to the disk.

### Usage Precautions
-   **Destructive Mode:** Use `mode: \"write\"` with extreme caution, as it will **permanently erase** existing file content. Verify the target path using `fs_ls` or `fs_pwd` before overwriting.
-   **Directory Existence:** The tool will fail if you attempt to create a file in a directory that does not exist. Ensure the parent directory structure is in place before calling `fs_write`.
-   **Concurrent Access:** Be mindful that other processes or VMs (e.g., your Debian/Ubuntu instances on Proxmox) might be accessing the same files. This tool does not implement mandatory file locking.
-   **Unix Permissions:** The tool operates under the permissions of the agent's process. It will fail with an error if it lacks write access to the target directory or file.
-   **Non-Text Data:** This tool is designed for UTF-8 strings. It is **not suitable for binary data** or complex non-text formats."
        )
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
                r#type: MessageType::System,
                contents: MessageContents::String("invalid parameter format".to_string()),
                tool_call_id: None,
            });
        };
        Ok(parameters)
    }
}
