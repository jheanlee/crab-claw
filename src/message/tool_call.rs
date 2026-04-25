use crate::message::message::{Message, MessageContents, MessageType};
use crate::tools::common::{Tool, ToolParameters};
use crate::tools::fs;
use crate::tools::fs::ls::Ls;
use crate::tools::fs::pwd::Pwd;
use crate::tools::fs::read::Read;
use crate::tools::fs::write::Write;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCallRequestMessage {
    pub id: String,
    pub tool: ToolKind,
    pub parameters: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ToolKind {
    Ls,
    Pwd,
    Read,
    Write,
    Unknown,
}

impl ToolKind {
    pub async fn run(&self, id: String, parameters: String) -> Message {
        match self {
            ToolKind::Ls => Ls::run(id, parameters).await,
            ToolKind::Pwd => Pwd::run(id, parameters).await,
            ToolKind::Read => Read::run(id, parameters).await,
            ToolKind::Write => Write::run(id, parameters).await,
            ToolKind::Unknown => Message {
                r#type: MessageType::ToolCallResponse,
                contents: MessageContents::String("tool name not found".to_string()),
            },
        }
    }

    pub fn from_string(name: String) -> Self {
        let ls = Ls::get_name();
        let pwd = Pwd::get_name();
        let read = Read::get_name();
        let write = Write::get_name();

        match name {
            s if s == ls => ToolKind::Ls,
            s if s == pwd => ToolKind::Pwd,
            s if s == read => ToolKind::Read,
            s if s == write => ToolKind::Write,
            _ => ToolKind::Unknown,
        }
    }
}
