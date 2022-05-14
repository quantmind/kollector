pub mod orderbook {
    tonic::include_proto!("orderbook");
}
use async_channel::{unbounded, Receiver, Sender};
use futures_util::Stream;
use orderbook::{orderbook_aggregator_server as obs, Empty, Summary};
use std::{net::ToSocketAddrs, pin::Pin};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};

type BookSummaryResult<T> = Result<Response<T>, Status>;

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
