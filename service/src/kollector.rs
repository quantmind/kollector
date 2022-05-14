use crate::grpc::{serve_grpc, OrderbookAggregator};
use common::{Context, InnerMessage};
use gateways::Gateway;
use slog::{error, info, warn};
use std::collections::HashMap;

/// The Kollector is the main the main service
pub struct Kollector {
    context: Context<InnerMessage>,
    gateways: HashMap<String, Box<dyn Gateway>>,
    grpc: OrderbookAggregator,
}

impl Default for Kollector {
    /// Create a new Kollector service
    fn default() -> Self {
        Self {
            context: Context::<InnerMessage>::new("kollector", None),
            gateways: HashMap::new(),
            grpc: OrderbookAggregator::new(),
        }
    }
}

impl Kollector {
    /// Spawn a websocket collector
    ///
    /// This method should be called before running the service
    pub fn spawn_gateway(&mut self, gateway: Box<dyn Gateway>) {
        // tasks for websockets streaming from the exchange
        //self.gateways.insert(gateway.name(), gateway);
    }

    /// Spawn the grpc server
    ///
    /// This method should be called before running the service
    pub fn spawn_grpc(&self) {
        serve_grpc(self.grpc.clone());
    }

    /// Main coroutine
    ///
    /// This coroutine runs the main part of the kollector service
    pub async fn run(&self, pair: &str) {
        let context = self.context.clone();

        // handle shutdown
        let signal = self.context.clone();
        ctrlc::set_handler(move || {
            signal.sender.try_send(InnerMessage::Exit).unwrap();
        })
        .expect("Error setting Ctrl-C handler");

        info!(context.logger, "start {}", context.name);
        loop {
            match context.receiver.recv().await {
                Ok(InnerMessage::Failure) => {
                    warn!(context.logger, "exit main worker after failure");
                    return;
                }
                Ok(InnerMessage::Exit) => {
                    warn!(context.logger, "exit main worker");
                    return;
                }
                // websocket payload
                Ok(InnerMessage::WsPayload(ws_payload)) => {
                    match self.gateways.get(&ws_payload.name) {
                        Some(gw) => {
                            gw.on_websocket_message(ws_payload.value);
                        }
                        None => {
                            error!(
                                context.logger,
                                "message from an unknown gateway {}", ws_payload.name
                            );
                        }
                    };
                }
                Ok(_) => {
                    // skip any other message
                }
                //
                Err(err) => {
                    error!(
                        context.logger,
                        "Main loop could not receive message: {}", err
                    );
                    return;
                }
            }
        }
    }
}
