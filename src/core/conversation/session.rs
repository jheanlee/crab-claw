use crate::llm::llm_provider::LLMProvider;
use crate::message::message::{Message, MessageContents, MessageType};
use crate::message::tool_call::ToolKind;
use nanoid::nanoid;
use std::future::pending;
use std::sync::Arc;
use tokio::select;
use tokio::sync::{RwLock, mpsc, watch};
use tokio::task::{JoinError, JoinHandle};
use tokio_util::task::JoinMap;

pub struct ConversationSession {
    pub provider: Arc<RwLock<Arc<Box<dyn LLMProvider>>>>,
    pub message_history: Arc<RwLock<Vec<Message>>>,
    pub tools: Arc<RwLock<Vec<ToolKind>>>,
    pub llm_running_rx: watch::Receiver<bool>,
    pub message_tx: mpsc::Sender<Message>,
    conversation_handler: JoinHandle<()>,
}

impl ConversationSession {
    pub fn new(
        provider: Box<dyn LLMProvider>,
        message_history: Vec<Message>,
        tools: Vec<ToolKind>,
    ) -> Self {
        let provider = Arc::new(RwLock::new(Arc::new(provider)));
        let message_history = Arc::new(RwLock::new(message_history));
        let tools = Arc::new(RwLock::new(tools));
        let (llm_running_tx, llm_running_rx) = watch::channel(false);
        let (message_tx, message_rx) = mpsc::channel(64);

        ConversationSession {
            provider: provider.clone(),
            message_history: message_history.clone(),
            tools: tools.clone(),
            llm_running_rx,
            message_tx,
            conversation_handler: tokio::spawn(Self::handler(
                provider,
                message_history,
                tools,
                llm_running_tx,
                message_rx,
            )),
        }
    }

    async fn handler(
        provider: Arc<RwLock<Arc<Box<dyn LLMProvider>>>>,
        message_history: Arc<RwLock<Vec<Message>>>,
        tools: Arc<RwLock<Vec<ToolKind>>>,
        llm_running_tx: watch::Sender<bool>,
        mut message_rx: mpsc::Receiver<Message>,
    ) {
        let mut tasks = JoinMap::new();
        let mut llm_call: Option<JoinHandle<Result<Message, crate::llm::error::Error>>> = None;
        let mut message_queued = false;

        loop {
            select! {
                llm_call_res = async {
                    match llm_call.as_mut() {
                        Some(llm_call) => llm_call.await,
                        None => pending().await
                    }
                }, if llm_call.is_some() => {
                    llm_call = None;

                    if let Some(message) = Self::llm_response_handler(llm_call_res) {
                        //  TODO: output
                        //  TODO: check reasoning models
                        match message.r#type {
                            MessageType::ToolCallRequest => {
                                match message.contents {
                                    MessageContents::String(_) => {},
                                    MessageContents::ToolCallRequests(ref requests) => {
                                        for request in requests {
                                            let request_clone = request.clone();
                                            let salt = nanoid!();
                                            tasks.spawn(
                                                request_clone.id.clone() + salt.as_str(),
                                                async move {
                                                    request_clone.tool.run(
                                                        request_clone.id,
                                                        request_clone.parameters
                                                    )
                                                    .await
                                                }
                                            );
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        message_history.write().await.push(message);
                    }
                }
                _ = async {}, if message_queued && tasks.is_empty() && llm_call.is_none() => {
                    message_queued = false;

                    let provider_clone = provider.read().await.clone();
                    let message_history_clone = message_history.read().await.clone();
                    let tools_clone = tools.read().await.clone();
                    let llm_running_tx_clone = llm_running_tx.clone();

                    llm_call = Some(tokio::spawn(async move {
                        llm_running_tx_clone.send_replace(true);
                        let res = provider_clone.request(message_history_clone, tools_clone).await;
                        llm_running_tx_clone.send_replace(false);
                        res
                    }));
                }
                Some((_, message)) = tasks.join_next(), if !tasks.is_empty() => {
                    match message {
                        Ok(message) => {
                            message_history.write().await.push(message);
                            message_queued = true;
                            //  TODO: limit tool call count to avoid infinite loops
                            //  TODO: log, output
                        }
                        Err(error) => {
                            //  TODO: log task panicked
                        }
                    }
                }
                message = message_rx.recv() => {
                    let Some(message) = message else {
                        break;
                    };

                    //  TODO: log message
                    //  TODO: output to client
                    message_history.write().await.push(message);
                    message_queued = true;
                }
            }
        }
    }

    fn llm_response_handler(
        response: Result<Result<Message, crate::llm::error::Error>, JoinError>,
    ) -> Option<Message> {
        match response {
            Ok(Ok(message)) => Some(message),
            Ok(Err(error)) => {
                match error {
                    crate::llm::error::Error::InvalidChoiceCount => Some(Message {
                        r#type: MessageType::System,
                        contents: MessageContents::String(
                            "invalid choice count received in the response".to_string(),
                        ),
                    }),
                    error => {
                        //  TODO: log error
                        //  TODO: output error
                        None
                    }
                }
            }
            Err(error) => {
                //  TODO: log error
                //  TODO: output error
                None
            }
        }
    }
}
