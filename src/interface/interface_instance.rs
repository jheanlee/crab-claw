use crate::conversation_message::message::Message;
use crate::interface::error::Error;
use async_trait::async_trait;
use tokio::sync::{broadcast, mpsc, watch};

#[async_trait]
pub trait InterfaceInstance {
    fn new(
        message_rx: broadcast::Receiver<Message>,
        user_message_tx: mpsc::Sender<Message>,
        llm_running_rx: watch::Receiver<bool>,
    ) -> Self;

    async fn run(&mut self) -> Result<(), Error>;
    async fn stop(&self);
}
