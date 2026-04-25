use crate::api::llm::openai::completion::common::{
    OpenAICompletionMessage, OpenAICompletionMessageRole, OpenAICompletionToolCall,
    OpenAICompletionToolFunctionCall,
};
use crate::api::llm::openai::completion::request::OpenAIToolChoice::Simple;
use crate::api::llm::openai::completion::request::{
    OpenAICompletionRequest, OpenAIFunction, OpenAISimpleToolChoice, OpenAITool,
};
use crate::api::llm::openai::completion::response::OpenAICompletionResponse;
use crate::api::llm::openai::completion::response::OpenAICompletionResponseChoiceFinishReason::ToolCalls;
use crate::tools::common::Tool;
use crate::tools::fs::ls::{Ls, LsParameters};
use crate::tools::fs::pwd::{Pwd, PwdParameters};
use crate::tools::fs::read::{Read, ReadParameters};
use crate::tools::fs::write::{Write, WriteMode, WriteParameters};
use serde_json::{json, to_string, to_value};

mod api;
mod llm;
mod message;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // let tools = vec![Ls.into(), Pwd.into(), Read.into(), Write.into()];
    //
    // let request = OpenAICompletionRequest {
    //     model: "claw-chat".to_string(),
    //     messages: vec![
    //         OpenAICompletionMessage {
    //             role: OpenAICompletionMessageRole::System,
    //             content: Some("You are a helpful assistant. Use the provided tool outputs to answer the user's question directly. The history of the conversation between you and the user as well as tool calls is provided. Only use tools if the user explicitly asks for file operations or information you do not have.".to_string()),
    //             tool_calls: None,
    //         },
    //         OpenAICompletionMessage {
    //             role: OpenAICompletionMessageRole::User,
    //             content: Some("Tell me which files I have in the current directory.".to_string()),
    //             tool_calls: None,
    //         },
    //         OpenAICompletionMessage {
    //             role: OpenAICompletionMessageRole::Assistant,
    //             content: None,
    //             tool_calls:Some(vec![OpenAICompletionToolCall { id: "call_72c2277a-25bb-4935-9ff0-992cde9e4451".to_string(), r#type: "function".to_string(), function: OpenAICompletionToolFunctionCall { name: "fs_pwd".to_string(), arguments: String::from("{}") } }])
    //         },
    //         OpenAICompletionMessage {
    //             role: OpenAICompletionMessageRole::Tool,
    //             content: Some(to_string(&Pwd::run("call_72c2277a-25bb-4935-9ff0-992cde9e4451".to_string(), PwdParameters{}).await)?),
    //             tool_calls: None
    //         },
    //         OpenAICompletionMessage {
    //             role: OpenAICompletionMessageRole::Assistant,
    //             content: None,
    //             tool_calls:Some(vec![OpenAICompletionToolCall { id: "call_a982e1d2-a9ad-4c5a-8af0-ca9d3810f3a4".to_string(), r#type: "function".to_string(), function: OpenAICompletionToolFunctionCall { name: "fs_ls".to_string(), arguments: to_string(&LsParameters {path: "/Users/jheanlee/Projects/crab-claw".to_string(), detailed: false})? } }])
    //         },
    //         OpenAICompletionMessage {
    //             role: OpenAICompletionMessageRole::Tool,
    //             content: Some(to_string(&Ls::run("call_a982e1d2-a9ad-4c5a-8af0-ca9d3810f3a4".to_string(), LsParameters {path: "/Users/jheanlee/Projects/crab-claw".to_string(), detailed: false}).await)?),
    //             tool_calls: None,
    //         }
    //     ],
    //     max_completion_tokens: None,
    //     max_tokens: None,
    //     tools: Some(tools),
    //     tool_choice: Some(Simple(OpenAISimpleToolChoice::Auto)),
    //     parallel_tool_calls: None,
    //     temperature: None,
    // };
    //
    // let client = reqwest::Client::new();
    // println!("{:?}", to_string(&request)?);
    // let res = client
    //     .post("http://localhost:4000/v1/chat/completions")
    //     .body(to_string(&request)?)
    //     .send()
    //     .await?;
    //
    // let res_bytes = res.bytes().await?;
    // println!("\n{}", String::from_utf8_lossy(res_bytes.as_ref()));
    // let res_parsed = serde_json::from_slice::<OpenAICompletionResponse>(&res_bytes)?;
    //
    // println!("\n{:?}", res_parsed);
    //
    // println!("\n{}", to_string(&res_parsed)?);

    Ok(())
}
