pub mod orderbook {
    tonic::include_proto!("orderbook");
}
use async_channel::{unbounded, Receiver, Sender};
use common::{bid_ask_spread, Book, L2};
use futures_util::Stream;
use orderbook::{orderbook_aggregator_server as obs, Empty, Level, Summary};
use rust_decimal::prelude::*;
use std::collections::HashMap;
use std::{net::ToSocketAddrs, pin::Pin};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};

pub type AssetBooks = HashMap<String, Book>;
type BookSummaryResult<T> = Result<Response<T>, Status>;

/// Orderbook Aggregator GRPC server
///
/// This struct implements the BookSummary streamying method for the GRPC server.
/// The server receive messages from the main application and broadcast them
/// to grpc upstream clients.
#[derive(Debug, Clone)]
pub struct OrderbookAggregator {
    /// use this to send messages to the Orderbook Aggregator service
    pub inbox: Sender<Summary>,
    // internal message receiver
    receiver: Receiver<Summary>,
}

/// Start serving the GRPC
pub fn serve_grpc(server: OrderbookAggregator) {
    tokio::spawn(async move {
        Server::builder()
            .add_service(obs::OrderbookAggregatorServer::new(server))
            .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
            .await
            .unwrap();
    });
}

/// Extract the book Summary protobuf message from asset order books from exchanges
pub fn book_summary(asset_books: &AssetBooks, max_depth: usize) -> Summary {
    let mut summary = Summary::default();
    let mut best_ask: Option<Decimal> = None;
    let mut best_bid: Option<Decimal> = None;
    for (exchange, book) in asset_books.iter() {
        best_ask = update_summary_side(
            &mut summary.asks,
            &book.asks,
            &exchange,
            max_depth,
            best_ask,
        );
        best_bid = update_summary_side(
            &mut summary.bids,
            &book.bids,
            &exchange,
            max_depth,
            best_bid,
        );
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
    max_depth: usize,
    best_price: Option<Decimal>,
) -> Option<Decimal> {
    for (i, (price, volume)) in book_side.iter().enumerate() {
        if i >= max_depth {
            break;
        }
        levels.push(Level {
            exchange: exchange.to_owned(),
            price: price.to_f64().unwrap(),
            amount: volume.to_f64().unwrap(),
        });
    }
    book_side.best_of(best_price)
}

impl OrderbookAggregator {
    /// create a new OrderbookAggregator server
    pub fn new() -> Self {
        let (inbox, receiver) = unbounded();
        Self { inbox, receiver }
    }
}

#[tonic::async_trait]
impl obs::OrderbookAggregator for OrderbookAggregator {
    type BookSummaryStream = Pin<Box<dyn Stream<Item = Result<Summary, Status>> + Send>>;

    async fn book_summary(&self, _: Request<Empty>) -> BookSummaryResult<Self::BookSummaryStream> {
        // get a new receiver for this connection
        let mut message_receiver = self.receiver.clone();

        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(message) = message_receiver.next().await {
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
            println!("\tclient disconnected");
        });

        let output_stream = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(output_stream) as Self::BookSummaryStream
        ))
    }
}
