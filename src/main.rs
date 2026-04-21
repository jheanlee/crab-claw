use crate::api::llm::openai::completion::common::{
    OpenAICompletionMessage, OpenAICompletionMessageRole,
};
use crate::api::llm::openai::completion::request::OpenAIToolChoice::Simple;
use crate::api::llm::openai::completion::request::{
    OpenAICompletionRequest, OpenAIFunction, OpenAISimpleToolChoice, OpenAITool,
};
use crate::api::llm::openai::completion::response::OpenAICompletionResponse;
use serde_json::{json, to_string};

mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let request = OpenAICompletionRequest {
        model: "claw-chat".to_string(),
        messages: vec![
            OpenAICompletionMessage {
                role: OpenAICompletionMessageRole::System,
                content: Some("Follow the user's instructions. Only use tools if the user explicitly asks for file operations or information you do not have.".to_string()),
                tool_calls: None,
            },
            OpenAICompletionMessage {
                role: OpenAICompletionMessageRole::User,
                content: Some("Hello!".to_string()),
                tool_calls: None,
            },
        ],
        max_completion_tokens: None,
        max_tokens: None,
        tools: Some(vec![
            OpenAITool {
                r#type: "function".to_string(),
                function: OpenAIFunction {
                    name: "fs_ls".to_string(),
                    description: Some("lists files in a directory".to_string()),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "path": {
                                "type": "string",
                                "description": "the path of the directory to list"
                            },
                            "recursive": {
                                "type": "bool",
                                "description": "(default false) whether to list the files recursively"
                            }
                        },
                        "required": ["path"],
                        "additionalProperties": false
                    }),
                    strict: Some(true),
                },
            },
            OpenAITool {
                r#type: "function".to_string(),
                function: OpenAIFunction {
                    name: "fs_open".to_string(),
                    description: Some("opens a file".to_string()),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "path": {
                                "type": "string",
                                "description": "the path to the file to open"
                            }
                        },
                        "required": ["path"],
                        "additionalProperties": false
                    }),
                    strict: Some(true),
                },
            },
        ]),
        tool_choice: Some(Simple(OpenAISimpleToolChoice::Auto)),
        parallel_tool_calls: None,
        temperature: None,
    };

    let client = reqwest::Client::new();
    println!("{:?}", to_string(&request)?);
    let res = client
        .post("http://localhost:4000/v1/chat/completions")
        .body(to_string(&request)?)
        .send()
        .await?;

    let res_parsed = serde_json::from_slice::<OpenAICompletionResponse>(&res.bytes().await?)?;

    println!("\n{:?}", res_parsed);

    println!("\n{}", to_string(&res_parsed)?);
    Ok(())
}
