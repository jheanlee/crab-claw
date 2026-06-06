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

use crate::conversation_message::message::{Message, MessageContents, MessageType};
use crate::tools::common::{
    Tool, ToolCallResponse, ToolCallResponseStatus, ToolParameterSchema, ToolParameters,
};
use crate::tools::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string, to_value};
use std::env;

pub struct Pwd;

impl Pwd {
    pub fn new() -> Self {
        Pwd {}
    }

    async fn _run(id: String, _parameters: PwdParameters) -> Result<Message, Error> {
        let pwd = env::current_dir()?;

        Ok(Message {
            r#type: MessageType::ToolCallResponse,
            contents: MessageContents::String(to_string(&ToolCallResponse {
                id: id.clone(),
                name: Self::get_name(),
                status: ToolCallResponseStatus::Success,
                content: to_value(pwd)?,
            })?),
            tool_call_id: Some(id),
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
        String::from("fs_pwd")
    }

    fn get_descriptions() -> String {
        String::from(
            "## Tool Name: `fs_pwd`
### Description
Returns the absolute path of the current working directory for the environment in which the agent is operating.

### Use Cases
The agent should use this tool when:
-   **Context Discovery:** It needs to establish where it is within the file system before performing file operations.
-   **Path Resolution:** It needs to resolve relative paths into absolute paths.
-   **Verification:** It needs to confirm that a previous directory change (e.g., via a `cd` command) was successful and that it is in the expected location.

### Behavior
-   **Input:** None (requires an empty parameters object `{}`).
-   **Output:** A string representing the current absolute path (e.g., `/home/user/project` or `/Users/name/work`).

### Usage Precautions
-   **Statelessness:** If the agent is running in a distributed or serverless environment, the current working directory might reset between different tool calls or conversation turns. Do not assume the directory remains persistent without verifying the environment's architecture.
-   **Symbolic Links:** The tool returns the actual current directory. Be aware that if the agent entered a directory via a symbolic link, `fs_pwd` may return the physical path rather than the logical path, depending on how the underlying Rust `std::env::current_dir()` handles the specific filesystem mount.
-   **Permissions:** While unlikely for the `pwd` action itself, the agent may have permission to be in a directory but lack permission to read its parent, which can occasionally cause issues in path canonicalization."
        )
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
pub struct PwdParameters {}

impl ToolParameters for PwdParameters {
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
