use crate::interface::interface_instance::InterfaceInstance;

mod api;
mod conversation_message;
mod core;
mod interface;
mod llm;
mod messaging;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    Ok(())
}
