use crate::grpc::{book_summary, serve_grpc, AssetBooks, OrderbookAggregator};
use crate::http::start_http_service;
use common::{wrap_result, Book, Context, InnerMessage};
use gateways::{Gateway, WsUpdate};
use slog::{error, info, warn};
use std::collections::{HashMap, HashSet};

/// The Kollector is the main the main service
pub struct Kollector {
    pub context: Context<InnerMessage>,
    pub max_depth: usize,
    gateways: HashMap<String, Box<dyn Gateway>>,
    books: HashMap<String, AssetBooks>,
    grpc: Option<OrderbookAggregator>,
}

impl Kollector {
    /// Create a new Kollector service
    pub fn new(max_depth: usize) -> Self {
        Self {
            context: Context::<InnerMessage>::new("kollector", None),
            gateways: HashMap::new(),
            books: HashMap::new(),
            grpc: None,
            max_depth,
        }
    }

    /// Spawn a gateway
    ///
    /// This method add a new gateway to the gateway's hashmap and
    /// start the websocket coroutine which connect to the exchange.
    ///
    /// This method should be called before running the service.
    pub fn spawn_gateway(&mut self, gateway: Box<dyn Gateway>) {
        gateway.setup();
        let ws = gateway.ws_consumer();
        self.gateways.insert(gateway.name().to_owned(), gateway);
        tokio::spawn(async move {
            let result = ws.run().await;
            wrap_result(&ws.context, result).await;
        });
    }

    /// Spawn the grpc server
    ///
    /// This method should be called before running the service
    pub fn spawn_grpc(&mut self) {
        let grpc = OrderbookAggregator::default();
        self.grpc = Some(grpc.clone());
        serve_grpc(grpc);
    }

    /// Add web service
    ///
    /// Add a web service for prometheus metrics and k8s liveness probe
    pub fn spawn_http(&self) {
        let service = self.context.clone();
        tokio::spawn(async move {
            start_http_service(service).await;
        });
    }

    /// Add Ctrl-C handler
    pub fn handle_ctrlc(&self) {
        // handle shutdown
        let signal = self.context.clone();
        ctrlc::set_handler(move || {
            signal.sender.try_send(InnerMessage::Exit).unwrap();
        })
        .expect("Error setting Ctrl-C handler");
    }

    /// Main coroutine
    ///
    /// This coroutine runs the main part of the kollector service
    pub async fn run(&mut self, pairs: &str) {
        let context = self.context.clone();
        let assets: Vec<String> = HashSet::<String>::from_iter(pairs.split(',').map(String::from))
            .into_iter()
            .collect();

        for gateway in self.gateways.values_mut() {
            info!(
                context.logger,
                "subscribe to {} {:?}",
                gateway.name(),
                assets
            );
            gateway.subscribe(&assets);
        }

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
                //
                Ok(InnerMessage::BookSnapshot(snapshot)) => {
                    let name = snapshot.name.to_owned();
                    match self.gateways.get_mut(&name) {
                        Some(gw) => {
                            if let Some(book) = gw.on_book_snapshot(snapshot) {
                                self.update_book(&name, book);
                            }
                        }
                        None => {
                            error!(context.logger, "snapshot from an unknown gateway {}", name);
                        }
                    }
                }
                // websocket payload
                Ok(InnerMessage::WsPayload(ws_payload)) => {
                    match self.gateways.get_mut(&ws_payload.name) {
                        Some(gw) => {
                            if let Some(update) = gw.on_websocket_message(ws_payload.value) {
                                match update {
                                    WsUpdate::Book(book) => {
                                        self.update_book(&ws_payload.name, book);
                                    }
                                }
                            }
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

    fn update_book(&mut self, name: &str, book: Book) {
        let asset = book.asset.to_owned();
        let asset_books = self
            .books
            .entry(asset.to_owned())
            .or_insert_with(HashMap::new);
        asset_books.insert(name.to_owned(), book);
        let summary = book_summary(asset_books);
        // broadcast the summary to listeners (for now grpc only)
        if let Some(grpc) = &self.grpc {
            grpc.context.try_send((asset, summary));
        }
    }
}
