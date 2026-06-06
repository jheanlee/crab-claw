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

use crate::api::llm::openai::completion::request::{OpenAIFunction, OpenAITool};
use crate::tools::common::Tool;
use serde_json::to_value;

pub trait IntoOpenAITool: Tool {
    fn into_openai_tool(self) -> OpenAITool;
}

impl<T: Tool> IntoOpenAITool for T {
    fn into_openai_tool(self) -> OpenAITool {
        OpenAITool {
            r#type: "function".to_string(),
            function: OpenAIFunction {
                name: T::get_name(),
                description: Some(T::get_descriptions()),
                parameters: to_value(T::get_parameter_schema()).expect("bad tool parameter schema"),
                strict: Some(T::get_strict()),
            },
        }
    }
}

impl<T: Tool> From<T> for OpenAITool {
    fn from(value: T) -> Self {
        value.into_openai_tool()
    }
}
