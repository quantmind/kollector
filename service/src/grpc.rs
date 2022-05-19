pub mod orderbook {
    tonic::include_proto!("orderbook");
}
use common::{bid_ask_spread, Book, Context, L2, wrap_result, WorkerContext};
use futures_util::Stream;
use orderbook::{orderbook_aggregator_server as obs, Empty, Level, Summary};
use rust_decimal::prelude::*;
use slog::info;
use std::collections::HashMap;
use std::{net::ToSocketAddrs, pin::Pin};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};

/// An hashmap mapping an exchange name with the orderbook for a given asset
pub type AssetBooks = HashMap<String, Book>;
type BookSummaryResult<T> = Result<Response<T>, Status>;
type GrpcContext = Context<(String, Summary)>;

/// Orderbook Aggregator GRPC server
///
/// This struct implements the BookSummary streamying method for the GRPC server.
/// The server receive messages from the main application and broadcast them
/// to grpc upstream clients.
#[derive(Clone)]
pub struct OrderbookAggregator {
    /// use this to send messages to the Orderbook Aggregator service
    pub context: GrpcContext,
}

/// Start serving the GRPC
///
/// The port is configured via the `app_grpc_port` environment variable and defaults to 50060.
pub fn serve_grpc(server: OrderbookAggregator, context: &WorkerContext) {
    let ctx = context.clone();
    tokio::spawn(async move {
        let host: String = server
            .context
            .get_or("app_grpc_host", "[::1]".to_owned())
            .expect("GRPC host");
        let port: u16 = server
            .context
            .get_or("app_grpc_port", 50060)
            .expect("GRPC port");
        let addr = format!("{}:{}", host, port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();
        info!(server.context.logger, "start the GRPC server {}", addr);
        let result = Server::builder()
            .accept_http1(true)
            .add_service(obs::OrderbookAggregatorServer::new(server))
            .serve(addr)
            .await.map_err(anyhow::Error::new);
        wrap_result(&ctx, result).await;
    });
}

/// Extract the book Summary protobuf message from asset order books from exchanges
pub fn book_summary(asset_books: &AssetBooks) -> Summary {
    let mut summary = Summary::default();
    let mut best_ask: Option<Decimal> = None;
    let mut best_bid: Option<Decimal> = None;
    for (exchange, book) in asset_books.iter() {
        best_ask = update_summary_side(&mut summary.asks, &book.asks, exchange, best_ask);
        best_bid = update_summary_side(&mut summary.bids, &book.bids, exchange, best_bid);
    }
    summary.spread = bid_ask_spread(best_bid, best_ask)
        .unwrap_or(Decimal::ZERO)
        .to_f64()
        .unwrap();
    summary
}

fn update_summary_side(
    levels: &mut Vec<Level>,
    book_side: &L2,
    exchange: &str,
    best_price: Option<Decimal>,
) -> Option<Decimal> {
    for (price, volume) in book_side.iter() {
        levels.push(Level {
            exchange: exchange.to_owned(),
            price: price.to_f64().unwrap(),
            amount: volume.to_f64().unwrap(),
        });
    }
    book_side.best_of(best_price)
}

impl Default for OrderbookAggregator {
    /// create a new OrderbookAggregator server
    fn default() -> Self {
        Self {
            context: GrpcContext::new("grpc", None),
        }
    }
}

#[tonic::async_trait]
impl obs::OrderbookAggregator for OrderbookAggregator {
    type BookSummaryStream = Pin<Box<dyn Stream<Item = Result<Summary, Status>> + Send>>;

    async fn book_summary(&self, _: Request<Empty>) -> BookSummaryResult<Self::BookSummaryStream> {
        // get a new receiver for this connection
        let mut context = self.context.clone();
        info!(context.logger, "new connection");

        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            while let Some((_, message)) = context.receiver.next().await {
                match tx.send(Result::<_, Status>::Ok(message)).await {
                    Ok(_) => {
                        // item (server response) was queued to be send to client
                    }
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
            }
            info!(context.logger, "client disconnected");
        });

        let output_stream = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(output_stream) as Self::BookSummaryStream
        ))
    }
}
