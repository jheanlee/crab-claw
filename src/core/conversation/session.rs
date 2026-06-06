/*
 * Copyright 2026 Jhe-An Lee
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::conversation_message::message::{Message, MessageContents, MessageType};
use crate::conversation_message::tool_call::ToolKind;
use crate::llm::llm_provider::LLMProvider;
use nanoid::nanoid;
use std::future::pending;
use std::sync::Arc;
use tokio::select;
use tokio::sync::{RwLock, broadcast, mpsc, watch};
use tokio::task::{JoinError, JoinHandle};
use tokio_util::task::JoinMap;
use tracing::{debug, trace, warn};

pub struct ConversationSession {
    llm_provider: Arc<RwLock<Arc<Box<dyn LLMProvider>>>>,
    message_history: Arc<RwLock<Vec<Message>>>,
    tools: Arc<RwLock<Vec<ToolKind>>>,
    pub llm_running_rx: watch::Receiver<bool>,
    pub user_message_tx: mpsc::Sender<Message>,
    message_tx: broadcast::Sender<Message>,
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
        let (message_tx, _) = broadcast::channel(64);
        let (user_message_tx, user_message_rx) = mpsc::channel(64);

        ConversationSession {
            llm_provider: provider.clone(),
            message_history: message_history.clone(),
            tools: tools.clone(),
            llm_running_rx,
            user_message_tx,
            message_tx: message_tx.clone(),
            conversation_handler: tokio::spawn(Self::handler(
                provider,
                message_history,
                tools,
                llm_running_tx,
                user_message_rx,
                message_tx,
            )),
        }
    }

    pub async fn get_provider(&self) -> Arc<Box<dyn LLMProvider>> {
        self.llm_provider.read().await.clone()
    }

    pub async fn set_provider(&mut self, provider: Box<dyn LLMProvider>) {
        *self.llm_provider.write().await = Arc::new(provider);
    }

    pub async fn get_message_history(&self) -> Vec<Message> {
        self.message_history.read().await.clone()
    }

    pub async fn get_tools(&self) -> Vec<ToolKind> {
        self.tools.read().await.clone()
    }

    pub async fn set_tools(&mut self, tools: Vec<ToolKind>) {
        *self.tools.write().await = tools;
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Message> {
        self.message_tx.subscribe()
    }

    async fn handler(
        provider: Arc<RwLock<Arc<Box<dyn LLMProvider>>>>,
        message_history: Arc<RwLock<Vec<Message>>>,
        tools: Arc<RwLock<Vec<ToolKind>>>,
        llm_running_tx: watch::Sender<bool>,
        mut user_message_rx: mpsc::Receiver<Message>,
        message_tx: broadcast::Sender<Message>,
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

                    if let Some(message) = Self::llm_response_handler(llm_call_res, message_tx.clone()) {
                        let _ = message_tx.send(message.clone());
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
                Some((id, message)) = tasks.join_next(), if !tasks.is_empty() => {
                    match message {
                        Ok(message) => {
                            let _ = message_tx.send(message.clone());
                            message_history.write().await.push(message);
                            message_queued = true;
                            //  TODO: limit tool call count to avoid infinite loops
                            trace!("Task {id} finished");
                        }
                        Err(error) => {
                            warn!("Task {id} panicked: {:?}", error);
                        }
                    }
                }
                message = user_message_rx.recv() => {
                    let Some(message) = message else {
                        break;
                    };

                    //  TODO: log message
                    let _ = message_tx.send(message.clone());
                    message_history.write().await.push(message);
                    message_queued = true;
                }
            }
        }
    }

    fn llm_response_handler(
        response: Result<Result<Message, crate::llm::error::Error>, JoinError>,
        response_tx: broadcast::Sender<Message>,
    ) -> Option<Message> {
        match response {
            Ok(Ok(message)) => Some(message),
            Ok(Err(error)) => match error {
                crate::llm::error::Error::InvalidChoiceCount => Some(Message {
                    r#type: MessageType::System,
                    contents: MessageContents::String(
                        "invalid choice count received in the response".to_string(),
                    ),
                    tool_call_id: None,
                }),
                error => {
                    debug!("LLM response failed: {:?}", error);
                    let _ = response_tx.send(Message {
                        r#type: MessageType::System,
                        contents: MessageContents::String(format!(
                            "An error has occurred: {:?}",
                            error
                        )),
                        tool_call_id: None,
                    });
                    None
                }
            },
            Err(error) => {
                trace!("LLM task panicked: {:?}", error);
                let _ = response_tx.send(Message {
                    r#type: MessageType::System,
                    contents: MessageContents::String(format!("An error has occurred {:?}", error)),
                    tool_call_id: None,
                });
                None
            }
        }
    }
}
