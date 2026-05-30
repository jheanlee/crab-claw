use crate::tools::fs::common::FsConfig;
use std::sync::LazyLock;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod api;
mod conversation_message;
mod core;
mod interface;
mod llm;
mod messaging;
mod tools;

pub static FS_CONFIG: LazyLock<FsConfig> = LazyLock::new(|| FsConfig {
    read_whitelist: Default::default(),
    write_whitelist: Default::default(),
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    Ok(())
}
