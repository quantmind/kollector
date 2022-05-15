use crate::backoff::{millis_sleep, Backoff};
use crate::worker::{Context, InnerMessage, WsInfo, WsPayload};
use anyhow::{Error, Result};
use async_channel::{unbounded, Receiver, Sender};
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use serde_json::{to_value, Value};
use slog::{debug, error, info, warn};
use std::time::{Duration, Instant};
use tokio::io;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};

/// Websocket consumer
///
/// Listen for new messages from websocket and write new commands into it
#[derive(Clone)]
pub struct WsConsumer {
    pub context: Context<InnerMessage>,
    pub sender: Sender<Value>,
    receiver: Receiver<Value>,
    heartbeat_sender: Sender<()>,
    heartbeat_receiver: Receiver<()>,
    heartbeat: Duration,
    ws_url: String,
}

impl WsConsumer {
    /// Create a new websocket consumer
    pub fn new(context: &Context<InnerMessage>, ws_url: &str) -> Self {
        let (sender, receiver) = unbounded();
        let (heartbeat_sender, heartbeat_receiver) = unbounded();
        WsConsumer {
            context: context.clone(),
            sender,
            receiver,
            heartbeat_sender,
            heartbeat_receiver,
            ws_url: ws_url.to_string(),
            heartbeat: Duration::from_millis(5000),
        }
    }

    pub fn info(&self) -> WsInfo {
        WsInfo {
            name: self.context.name.clone(),
            url: self.ws_url.clone(),
        }
    }

    /// schedule a write into the websocket
    pub fn write<T: Serialize>(&self, message: T) {
        self.sender.try_send(to_value(message).unwrap()).unwrap();
    }

    fn payload(&self, value: Value) -> WsPayload {
        WsPayload {
            name: self.context.name.clone(),
            url: self.ws_url.clone(),
            value,
        }
    }

    pub fn get_url(&self) -> &str {
        &self.ws_url
    }

    fn create_ws_request(&self) -> &str {
        &self.ws_url
    }

    // coroutine for consuming and writing messages into a websocket
    pub async fn run(&self) -> Result<()> {
        let mut backoff = Backoff::new(10, 1, 20, 2);
        let logger = self.context.logger.clone();

        // heartbeat to check health of connection
        let heartbeat_sender = self.heartbeat_sender.clone();
        let heartbeat = self.heartbeat;
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(heartbeat).await;
                heartbeat_sender.send(()).await.unwrap();
            }
        });

        // main loop with backoff-reconnection
        loop {
            match backoff.next() {
                Some(delay) => {
                    if delay > 0 {
                        info!(
                            logger,
                            "attempt {} to reconnect to websocket in {} seconds",
                            backoff.count(),
                            delay
                        );
                        millis_sleep(1000 * delay as u64).await;
                    }
                }
                None => {
                    return Err(Error::msg(format!(
                        "failed to reconnect to websocket after {} attempts",
                        backoff.count()
                    )));
                }
            }

            info!(logger, "connecting with websocket {}", &self.ws_url);
            let ws_stream = match connect_async(self.create_ws_request()).await {
                Ok((ws_stream, _)) => {
                    warn!(logger, "connected with websocket {}", &self.ws_url);
                    backoff.reset();
                    ws_stream
                }
                Err(err) => {
                    warn!(
                        logger,
                        "failed to connect with websocket {}: {}", &self.ws_url, err
                    );
                    continue;
                }
            };

            self.stream(ws_stream).await.unwrap_or_else(|err| {
                error!(logger, "{}", err);
            });

            self.context
                .send(InnerMessage::WsDisconnected(self.info()))
                .await;
        }
    }

    async fn stream<S>(&self, mut ws_stream: WebSocketStream<S>) -> Result<()>
    where
        S: io::AsyncRead + io::AsyncWrite + Unpin,
    {
        let logger = &self.context.logger;
        let url = &self.ws_url;
        let mut last_frame_instant = Instant::now();
        let mut num_messages_since_last_heartbeat = 0;
        // send connected message to the main application
        self.context
            .send(InnerMessage::WsConnected(self.info()))
            .await;
        loop {
            tokio::select! {
                // Handle stream of messages from exchange
                Some(frame) = ws_stream.next() => {
                    last_frame_instant = Instant::now();
                    num_messages_since_last_heartbeat += 1;
                    match frame {
                        Ok(Message::Text(ref text)) => {
                            let value: Value = match serde_json::from_str(text.as_ref()) {
                                Ok(value) => value,
                                Err(err) => {
                                    let mut context = String::from("malformed json message: ");
                                    context.push_str(text);
                                    return Err(Error::new(err).context(context));
                                }
                            };
                            // send websocket message to the main task
                            self.context.send(InnerMessage::WsPayload(self.payload(value))).await;
                        }
                        Ok(Message::Close(_)) =>{
                            warn!(logger, "received a close frame, stop streaming and reconnect");
                            return Ok(());
                        }
                        Ok(Message::Ping(msg)) =>{
                            info!(logger, "send a pong message after receiving a server ping");
                            ws_stream.send(Message::Pong(msg)).await?;
                        }
                        Ok(Message::Pong(_)) =>{
                            info!(logger, "got a pong message from server");
                        }
                        Ok(Message::Binary(_)) =>{
                            warn!(logger, "got a binary message from server - skip it");
                        }
                        Ok(Message::Frame(_)) =>{
                            warn!(logger, "got a raw frame message from server - skip it");
                        }
                        Err(err) => {
                            return Err(Error::msg(format!("error while streaming websocket: {}", err)));
                        }
                    };
                }
                // write new messages
                Ok(message) = self.receiver.recv() => {
                    debug!(logger, "write new message into websocket: {}", message);
                    ws_stream.send(Message::Text(message.to_string())).await?;
                }
                // heartbeat
                Ok(_) = self.heartbeat_receiver.recv() => {
                    if Instant::now() - last_frame_instant > self.heartbeat && num_messages_since_last_heartbeat == 0 {
                        warn!(logger, "{} no messages received since last heartbeat, exit receiving loop and reconnect", self.context.name);
                        return Ok(());
                    } else {
                        debug!(logger, "{} received {} messages since last heartbeat", url, num_messages_since_last_heartbeat);
                        num_messages_since_last_heartbeat = 0;
                    }
                }
            }
        }
    }
}
