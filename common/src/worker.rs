use crate::logging::init_logging;
use crate::orders::Book;
use anyhow::Result;
use async_channel::{unbounded, Receiver, Sender};
use config::builder::{ConfigBuilder, DefaultState};
use config::{Config, Environment};
use serde_json::Value;
use slog::{error, info, Logger};

pub type CfgBuilder = ConfigBuilder<DefaultState>;

/// Worker Context
///
///  A context is the basic configuration for a worker
#[derive(Clone)]
pub struct Context<T> {
    /// name of the worker
    pub name: String,
    /// configuration
    pub cfg: Config,
    /// logging
    pub logger: Logger,
    /// Use this to send messages to another worker
    pub sender: Sender<T>,
    /// Use this to receive messages from another worker
    pub receiver: Receiver<T>,
}

#[derive(Debug, Clone)]
pub struct WsInfo {
    pub name: String,
    pub url: String,
}

/// Websocket payload
///
/// This struct is used by the websocket consumer to communicate back to the main application
/// once a new websocket message arrives
#[derive(Debug, Clone)]
pub struct WsPayload {
    /// gateway name
    pub name: String,
    /// websocket url
    pub url: String,
    /// message payload
    pub value: Value,
}

/// Orderbook snapshot message
///
/// An orderbook snapshot is a representation of the order boo at a given time.
/// It can be used as the starting point of an in-memory order book.
#[derive(Debug, Clone)]
pub struct BookSnapshot {
    /// gateway name
    pub name: String,
    /// sequence number (use this to discard deltas with sequence prior to this)
    pub sequence: usize,
    /// the order book snapshot
    pub book: Book,
}

/// Internal message enum
///
/// This enum is used to send messages between different coroutines
#[derive(Debug, Clone)]
pub enum InnerMessage {
    /// heartbeat message
    Heartbeat,
    /// clean exit
    Exit,
    /// exit with failure
    Failure,
    /// websocket message
    WsConnected(WsInfo),
    /// websocket disconnect
    WsDisconnected(WsInfo),
    /// websocket payload
    WsPayload(WsPayload),
    /// Orderbook snapshot
    BookSnapshot(BookSnapshot),
}

pub fn create_config() -> CfgBuilder {
    Config::builder().add_source(Environment::default())
}

impl<T> Context<T> {
    pub fn new(name: &str, config: Option<Config>) -> Self {
        let (sender, receiver) = unbounded();
        let cfg = match config {
            Some(cfg) => cfg,
            None => create_config().build().expect("config"),
        };
        let logger = init_logging(&cfg).unwrap();
        Self {
            name: name.to_owned(),
            cfg,
            logger,
            sender,
            receiver,
        }
    }

    pub async fn send(&self, msg: T) {
        self.sender.send(msg).await.unwrap();
    }
}

pub async fn wrap_result(context: &Context<InnerMessage>, result: Result<()>) {
    match result {
        Ok(()) => {
            info!(context.logger, "{} - exited", context.name);
            context.send(InnerMessage::Exit).await;
        }
        Err(err) => {
            error!(
                context.logger,
                "{} - unexpected error - {}", context.name, err
            );
            context.send(InnerMessage::Failure).await;
        }
    };
}
