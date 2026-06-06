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

use crate::api::llm::openai::completion::common::OpenAICompletionMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAICompletionResponse {
    pub id: String,
    pub model: String,
    pub object: String, //  "chat.completion"
    pub created: u64,   //  timestamp
    pub choices: Vec<OpenAICompletionResponseChoice>,
    pub usage: OpenAICompletionResponseUsage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAICompletionResponseChoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<OpenAICompletionResponseChoiceFinishReason>,
    pub index: u32,
    pub message: OpenAICompletionMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OpenAICompletionResponseChoiceFinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAICompletionResponseUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
