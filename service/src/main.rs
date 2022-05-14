//! GRPC Service exposing streaming endpoint of order book updates
use clap::Parser;
use gateways::Binance;
use service::Kollector;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Currency pair
    #[clap(short, long)]
    pair: String,
}

#[tokio::main]
async fn main() {
    dotenv::from_path(".env").ok();
    let app = Args::parse();

    // create the service
    let mut kollector = Kollector::default();
    // spawn the GRPC server
    kollector.spawn_grpc();
    // spawn gateways
    kollector.spawn_gateway(Box::new(Binance::default()));
    //kollector.spawn_gateway(Bitstamp::default());
    // run the main application
    kollector.run(&app.pair).await;
}
