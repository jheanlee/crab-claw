mod api;
mod core;
mod llm;
mod message;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    Ok(())
}
