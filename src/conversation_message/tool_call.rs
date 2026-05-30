use crate::api::llm::openai::completion::request::OpenAITool;
use crate::conversation_message::message::{Message, MessageContents, MessageType};
use crate::tools::common::Tool;
use crate::tools::{env, fs};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCallRequestMessage {
    pub id: String,
    pub tool: ToolKind,
    pub parameters: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToolKind {
    FsLs,
    FsPwd,
    FsRead,
    FsWrite,
    EnvCurrentTime,
    Unknown,
}

impl ToolKind {
    pub async fn run(&self, id: String, parameters: String) -> Message {
        match self {
            ToolKind::FsLs => fs::ls::Ls::run(id, parameters).await,
            ToolKind::FsPwd => fs::pwd::Pwd::run(id, parameters).await,
            ToolKind::FsRead => fs::read::Read::run(id, parameters).await,
            ToolKind::FsWrite => fs::write::Write::run(id, parameters).await,
            ToolKind::EnvCurrentTime => env::current_time::CurrentTime::run(id, parameters).await,
            ToolKind::Unknown => Message {
                r#type: MessageType::ToolCallResponse,
                contents: MessageContents::String("tool name not found".to_string()),
                tool_call_id: Some(id),
            },
        }
    }

    pub fn from_string(name: String) -> Self {
        let ls = fs::ls::Ls::get_name();
        let pwd = fs::pwd::Pwd::get_name();
        let read = fs::read::Read::get_name();
        let write = fs::write::Write::get_name();
        let current_time = env::current_time::CurrentTime::get_name();

        match name {
            s if s == ls => ToolKind::FsLs,
            s if s == pwd => ToolKind::FsPwd,
            s if s == read => ToolKind::FsRead,
            s if s == write => ToolKind::FsWrite,
            s if s == current_time => ToolKind::EnvCurrentTime,
            _ => ToolKind::Unknown,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ToolKind::FsLs => fs::ls::Ls::get_name(),
            ToolKind::FsPwd => fs::pwd::Pwd::get_name(),
            ToolKind::FsRead => fs::read::Read::get_name(),
            ToolKind::FsWrite => fs::write::Write::get_name(),
            ToolKind::EnvCurrentTime => env::current_time::CurrentTime::get_name(),
            ToolKind::Unknown => "unknown".to_string(),
        }
    }
}

impl Into<OpenAITool> for ToolKind {
    fn into(self) -> OpenAITool {
        match self {
            ToolKind::FsLs => fs::ls::Ls::new().into(),
            ToolKind::FsPwd => fs::pwd::Pwd::new().into(),
            ToolKind::FsRead => fs::read::Read::new().into(),
            ToolKind::FsWrite => fs::write::Write::new().into(),
            ToolKind::EnvCurrentTime => env::current_time::CurrentTime::new().into(),
            ToolKind::Unknown => panic!("unknown should not be listed as an available tool"),
        }
    }
}
