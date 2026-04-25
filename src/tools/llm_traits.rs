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
