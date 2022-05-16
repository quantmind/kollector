use clap::Parser;
use gateways::{Binance, Bitstamp};
use service::Kollector;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Comma separated list of currency pairs
    #[clap(short, long)]
    pairs: String,

    /// Maximum number of order book levels to track
    #[clap(short, long, default_value_t = 10)]
    max_depth: usize,
}

#[macro_export]
macro_rules! run_gateway {
    ($kollector: ident, $Gateway: ident) => {
        $kollector.spawn_gateway(Box::new($Gateway::new(
            &$kollector.context,
            $kollector.max_depth,
        )))
    };
}

#[tokio::main]
async fn main() {
    dotenv::from_path(".env").ok();
    let app = Args::parse();

    // create the service
    let mut kollector = Kollector::new(app.max_depth);
    // add Ctrl-c handler
    kollector.handle_ctrlc();
    // spawn the GRPC server
    kollector.spawn_grpc();
    // spawn gateways
    run_gateway!(kollector, Binance);
    run_gateway!(kollector, Bitstamp);
    // run the main application
    kollector.run(&app.pairs).await;
}
