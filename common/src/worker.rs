use crate::logging::init_logging;
use async_channel::{unbounded, Receiver, Sender};
use config::builder::{ConfigBuilder, DefaultState};
use config::{Config, Environment};
use serde_json::Value;
use slog::Logger;

pub type CfgBuilder = ConfigBuilder<DefaultState>;

// A context is the basic configuration for a worker
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

#[derive(Debug, Clone)]
pub struct WsPayload {
    pub name: String,
    pub url: String,
    pub value: Value,
}

// Internal message enum
#[derive(Debug, Clone)]
pub enum InnerMessage {
    // heartbeat message
    Heartbeat,
    // clean exit
    Exit,
    // exit with failure
    Failure,
    // websocket message
    WsConnected(WsInfo),
    // websocket disconnect
    WsDisconnected(WsInfo),
    // websocket payload
    WsPayload(WsPayload),
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
