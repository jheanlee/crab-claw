use crate::message::message::{IntoMessage, Message, MessageContents, MessageType};
use crate::message::tool_call::{ToolCallRequestMessage, ToolKind};
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_str};

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
    pub tool_calls: Option<Vec<OpenAICompletionToolCall>>,
}

impl IntoMessage for OpenAICompletionMessage {
    fn into_message(self) -> Message {
        match self.role {
            OpenAICompletionMessageRole::System => Message {
                r#type: MessageType::System,
                contents: MessageContents::String(self.content.unwrap_or(String::new())),
            },
            OpenAICompletionMessageRole::User => Message {
                r#type: MessageType::User,
                contents: MessageContents::String(self.content.unwrap_or(String::new())),
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
                    }
                } else {
                    Message {
                        r#type: MessageType::LLM,
                        contents: MessageContents::String(self.content.unwrap_or(String::new())),
                    }
                }
            }
            OpenAICompletionMessageRole::Tool => Message {
                r#type: MessageType::ToolCallResponse,
                contents: MessageContents::String(self.content.unwrap_or(String::new())),
            },
            OpenAICompletionMessageRole::Function => {
                unimplemented!()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAICompletionToolCall {
    pub id: String,
    pub r#type: String, // Usually "function"
    pub function: OpenAICompletionToolFunctionCall,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAICompletionToolFunctionCall {
    pub name: String,
    pub arguments: String,
}
