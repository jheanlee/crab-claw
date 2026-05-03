use crate::messaging::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait MessagingProvider: Send + Sync {
    fn new() -> Self;
    async fn read_message_by_id(&self, message_id: String) -> Result<String, Error>;
    async fn start_typing(&self) -> Result<(), Error>;
    async fn stop_typing(&self) -> Result<(), Error>;
}
