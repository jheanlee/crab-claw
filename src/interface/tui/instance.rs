use crate::conversation_message::message::{Message, MessageContents, MessageType};
use crate::interface::error::Error;
use crate::interface::interface_instance::InterfaceInstance;
use crate::interface::tui::app::tui::TUI;
use async_trait::async_trait;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::backend::CrosstermBackend;
use ratatui::style::Style;
use ratatui::{Terminal, TerminalOptions, Viewport};
use serde_json::to_string;
use serenity::futures::StreamExt;
use tokio::select;
use tokio::sync::{broadcast, mpsc};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use tui_input::backend::crossterm::EventHandler;

pub struct TUIInstance {
    message_rx: broadcast::Receiver<Message>,
    user_message_tx: mpsc::Sender<Message>,
    llm_running_rx: tokio::sync::watch::Receiver<bool>,
    cancellation_token: CancellationToken,
}

impl TUIInstance {
    async fn handle_key_event(&self, tui: &mut TUI, key: event::KeyEvent) -> Result<(), Error> {
        match key.code {
            KeyCode::Enter => {
                let input_val = tui.input.value().to_string();
                if !input_val.is_empty() {
                    self.user_message_tx
                        .send(Message {
                            r#type: MessageType::User,
                            contents: MessageContents::String(input_val.clone()),
                        })
                        .await
                        .map_err(|_| Error::SendError)?;

                    tui.push_message(format!("> {}", input_val), Style::new().cyan())?;
                    tui.input.reset();
                }
            }
            _ => {
                tui.input.handle_event(&Event::Key(key));
            }
        }
        Ok(())
    }
}

#[async_trait]
impl InterfaceInstance for TUIInstance {
    fn new(
        message_rx: broadcast::Receiver<Message>,
        user_message_tx: mpsc::Sender<Message>,
        llm_running_rx: tokio::sync::watch::Receiver<bool>,
    ) -> Self {
        TUIInstance {
            message_rx,
            user_message_tx,
            llm_running_rx,
            cancellation_token: CancellationToken::new(),
        }
    }

    async fn run(&mut self) -> Result<(), Error> {
        crossterm::terminal::enable_raw_mode()?;
        let backend = CrosstermBackend::new(std::io::stdout());
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(3), // Height of your floating input
            },
        )?;
        let mut tui = TUI::new(terminal);

        let mut reader = event::EventStream::new();

        loop {
            if tui.draw().is_err() {
                error!("Unable to start tui");
                break;
            };

            select! {
                _ = self.cancellation_token.cancelled() => {}
                message = self.message_rx.recv() => {
                    match message {
                        Ok(message) => {
                            match message.r#type {
                                MessageType::LLM => {
                                    if let MessageContents::String(string) = message.contents {
                                        tui.push_message(string, Style::new())?;
                                    }
                                },
                                MessageType::ToolCallRequest => {
                                    if let MessageContents::ToolCallRequests(requests) = message.contents {
                                        for request in requests {
                                            tui.push_message(format!("ToolCall: {}", to_string(&request).unwrap_or(String::new())), Style::new().dark_gray())?;
                                        }
                                    }
                                },
                                MessageType::ToolCallResponse => {
                                    if let MessageContents::String(string) = message.contents {
                                        tui.push_message(format!("ToolResponse: {string}"), Style::new().dark_gray())?;
                                    }
                                },
                                _ => {}
                            };
                        }
                        Err(error) => {
                            info!("Conversation closed");
                            break;
                        }
                    }
                }
                event = reader.next() => {
                    match event {
                        Some(Ok(Event::Key(key))) => {
                            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                                self.cancellation_token.cancel();
                                break;
                            }
                            self.handle_key_event(&mut tui, key).await?;
                        }
                        _ => {}
                    }
                }

            }
        }
        ratatui::restore();
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    async fn stop(&self) {
        self.cancellation_token.cancel();
    }
}
