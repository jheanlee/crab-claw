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

use crate::conversation_message::message::{IntoMessage, Message, MessageContents, MessageType};
use crate::conversation_message::tool_call::{ToolCallRequestMessage, ToolKind};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OpenAICompletionMessageRole {
    System,
    User,
    Assistant,
    Tool,
    Function, //  Not implemented
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAICompletionMessage {
    pub role: OpenAICompletionMessageRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<OpenAICompletionToolCall>>,
}

impl IntoMessage for OpenAICompletionMessage {
    fn into_message(self) -> Message {
        match self.role {
            OpenAICompletionMessageRole::System => Message {
                r#type: MessageType::System,
                contents: MessageContents::String(self.content.unwrap_or(String::new())),
                tool_call_id: None,
            },
            OpenAICompletionMessageRole::User => Message {
                r#type: MessageType::User,
                contents: MessageContents::String(self.content.unwrap_or(String::new())),
                tool_call_id: None,
            },
            OpenAICompletionMessageRole::Assistant => {
                if self.tool_calls.is_some() {
                    Message {
                        r#type: MessageType::ToolCallRequest,
                        contents: MessageContents::ToolCallRequests({
                            self.tool_calls
                                .unwrap_or(Vec::new())
                                .iter()
                                .map(|call| ToolCallRequestMessage {
                                    id: call.id.clone(),
                                    tool: ToolKind::from_string(call.function.name.clone()),
                                    parameters: call.function.arguments.clone(),
                                })
                                .collect()
                        }),
                        tool_call_id: None,
                    }
                } else {
                    Message {
                        r#type: MessageType::LLM,
                        contents: MessageContents::String(self.content.unwrap_or(String::new())),
                        tool_call_id: None,
                    }
                }
            }
            OpenAICompletionMessageRole::Tool => Message {
                r#type: MessageType::ToolCallResponse,
                contents: MessageContents::String(self.content.unwrap_or(String::new())),
                tool_call_id: None,
            },
            OpenAICompletionMessageRole::Function => {
                unimplemented!()
            }
        }
    }
}

impl From<Message> for OpenAICompletionMessage {
    fn from(value: Message) -> Self {
        match value.r#type {
            MessageType::System => OpenAICompletionMessage {
                role: OpenAICompletionMessageRole::System,
                content: Some(value.contents.into()),
                tool_call_id: value.tool_call_id,
                tool_calls: None,
            },
            MessageType::User => OpenAICompletionMessage {
                role: OpenAICompletionMessageRole::User,
                content: Some(value.contents.into()),
                tool_call_id: value.tool_call_id,
                tool_calls: None,
            },
            MessageType::LLM => OpenAICompletionMessage {
                role: OpenAICompletionMessageRole::Assistant,
                content: Some(value.contents.into()),
                tool_call_id: value.tool_call_id,
                tool_calls: None,
            },
            MessageType::ToolCallRequest => OpenAICompletionMessage {
                role: OpenAICompletionMessageRole::Assistant,
                content: None,
                tool_call_id: value.tool_call_id,
                tool_calls: Some({
                    match value.contents {
                        MessageContents::String(_) => {
                            panic!("Tool call requests should never be a string")
                        }
                        MessageContents::ToolCallRequests(request) => request
                            .iter()
                            .map(|call| OpenAICompletionToolCall {
                                id: call.id.clone(),
                                r#type: "function".to_string(),
                                function: OpenAICompletionToolFunctionCall {
                                    name: call.tool.to_string(),
                                    arguments: call.parameters.clone(),
                                },
                            })
                            .collect(),
                    }
                }),
            },
            MessageType::ToolCallResponse => OpenAICompletionMessage {
                role: OpenAICompletionMessageRole::Tool,
                content: Some(value.contents.into()),
                tool_call_id: value.tool_call_id,
                tool_calls: None,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAICompletionToolCall {
    pub id: String,
    pub r#type: String, // Always "function"
    pub function: OpenAICompletionToolFunctionCall,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAICompletionToolFunctionCall {
    pub name: String,
    pub arguments: String,
}
