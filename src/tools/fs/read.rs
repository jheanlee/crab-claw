use crate::conversation_message::message::{Message, MessageContents, MessageType};
use crate::tools::common::{
    Tool, ToolCallResponse, ToolCallResponseStatus, ToolParameterSchema, ToolParameters,
};
use crate::tools::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string, to_value};
use tokio::fs::read_to_string;

pub struct Read;

impl Read {
    pub fn new() -> Self {
        Read {}
    }

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
            "## Tool Name: `fs_read`

### Description
Reads the entire content of a specified file and returns it as a UTF-8 encoded string.

### Use Cases
The agent should use this tool when:
-   **Code Review/Analysis:** It needs to examine the source code of a file to provide explanations or refactoring suggestions.
-   **Configuration Reading:** It needs to extract settings or environment variables from configuration files (e.g., `.env`, `Cargo.toml`, `settings.json`).
-   **Data Extraction:** It needs to retrieve information from text-based logs, documentation, or data files.
-   **Modification Prep:** It needs to see the current state of a file before applying edits or overwriting content.

### Parameters
-   `path` (string, **required**): The path to the file to be read. Supports absolute paths, relative paths, and tilde expansion (e.g., `~/.bashrc`).

### Behavior
-   **Encoding:** The tool strictly expects and returns **UTF-8** encoded text.
-   **Path Resolution:** Automatically expands `~` to the user's home directory.
-   **Asynchronous I/O:** Utilizes non-blocking I/O (via `tokio::fs`) to ensure the agent environment remains responsive during the read operation.

### Usage Precautions
-   **Token Management:** Reading very large files can consume a significant portion of the agent's context window (token limit). For large files, consider using alternative methods to \"peak\" at the file or process it in chunks if available.
-   **Binary Files:** **Warning: This tool is not suitable for reading binary files** (e.g., images, compiled executables, or compressed archives). Attempting to read non-UTF-8 data will result in an error or corrupted text.
-   **Memory Usage:** Because the tool reads the *entire* file into memory as a string, attempting to read multi-gigabyte files may lead to memory exhaustion or tool failure.
-   **Environment:** While cross-platform at the Rust level, the tool follows **Unix-like path conventions**. On systems like macOS or Debian, ensure the calling agent has the necessary read permissions for the target file.
-   **Security:** **Do not use this tool to read confidential files, private keys, or secrets (e.g., `.ssh/id_rsa`, `shadow`, or sensitive credential stores) unless explicitly authorized for a specific troubleshooting task.**"
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
