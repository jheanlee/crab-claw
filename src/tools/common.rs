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

use crate::conversation_message::message::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait Tool {
    async fn run(id: String, parameters: String) -> Message;
    fn get_name() -> String;
    fn get_descriptions() -> String;
    fn get_parameter_schema() -> ToolParameterSchema;
    fn get_strict() -> bool;
}

pub trait ToolParameters {
    fn from_string(raw_parameters: String) -> Result<Box<Self>, Message>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolParameterSchema {
    pub r#type: String,
    pub properties: Value,
    pub required: Vec<String>,
    pub additional_properties: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCallResponse {
    pub id: String,
    pub name: String,
    pub status: ToolCallResponseStatus,
    pub content: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToolCallResponseStatus {
    Success,
    Error,
}
