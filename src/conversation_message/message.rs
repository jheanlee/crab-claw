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

use crate::conversation_message::tool_call::ToolCallRequestMessage;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub r#type: MessageType,
    pub contents: MessageContents,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    System,
    User,
    LLM,
    ToolCallRequest,
    ToolCallResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageContents {
    String(String),
    ToolCallRequests(Vec<ToolCallRequestMessage>),
}

impl Into<String> for MessageContents {
    fn into(self) -> String {
        match self {
            MessageContents::String(string) => string,
            MessageContents::ToolCallRequests(requests) => {
                to_string(&requests).expect("serialisation failed")
            }
        }
    }
}

pub trait IntoMessage {
    fn into_message(self) -> Message;
}
