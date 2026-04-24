use std::net::SocketAddr;
use crate::api::llm::common::LLMProvider;

pub struct OpenAIProvider {
    pub model: String,
    pub api_base: SocketAddr,
    pub api_key: String,
}

impl LLMProvider for OpenAIProvider {}
