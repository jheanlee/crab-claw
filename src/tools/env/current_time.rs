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
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string};

pub struct CurrentTime {}

impl CurrentTime {
    pub fn new() -> Self {
        CurrentTime {}
    }
    async fn _run(id: String, _parameters: CurrentTimeParameters) -> Result<Message, Error> {
        let time = Local::now();

        Ok(Message {
            r#type: MessageType::ToolCallResponse,
            contents: MessageContents::String(to_string(&ToolCallResponse {
                id: id.clone(),
                name: Self::get_name(),
                status: ToolCallResponseStatus::Success,
                content: json!({
                    "local_time": time.to_rfc3339(),
                    "utc_time": time.to_utc().to_rfc3339()
                }),
            })?),
            tool_call_id: Some(id),
        })
    }
}

impl Tool for CurrentTime {
    async fn run(id: String, parameters: String) -> Message {
        match CurrentTimeParameters::from_string(parameters) {
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
        String::from("env_current_time")
    }

    fn get_descriptions() -> String {
        String::from(
            "## Tool Name: `env_current_time`
### Description
Returns the current local and UTC timestamps of the environment where the agent is operating.

### Use Cases
The agent should use this tool when:
-   **Temporal Awareness:** It needs to understand the current date or time to make scheduling, logging, or context-dependent decisions.
-   **Time Calculations:** It needs to compute durations, deadlines, or historical differences relative to the present moment.
-   **Data Synchronization:** It needs to timestamp file creation, API requests, or database records accurately.

### Behavior
-   **Input:** None (requires an empty parameters object `{}`).
-   **Output:** A JSON object containing the current ISO 8601 / RFC 3339 formatted strings for both `local_time` and `utc_time`.

### Usage Precautions
-   **Time Zone Discrepancies:** The `local_time` field reflects the system clock configuration of the environment where the tool executes, which might differ from the user's local timezone. Always inspect the offset provided in the timestamp string.
-   **Drift and Synchronization:** In serverless or distributed setups, minor clock drift might occur between different execution environments. Relying on sub-second precision for sequence synchronization across systems is discouraged."
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
pub struct CurrentTimeParameters {}

impl ToolParameters for CurrentTimeParameters {
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
