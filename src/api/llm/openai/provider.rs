use std::net::SocketAddr;

pub struct OpenAIProvider {
    pub model: String,
    pub api_base: SocketAddr,
    pub api_key: String,
}

impl OpenAIProvider {}
