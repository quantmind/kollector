//! Connect and map remote exchanges messages
mod binance;
mod bitstamp;
mod gateway;

pub use self::binance::Binance;
pub use self::bitstamp::Bitstamp;
pub use self::gateway::*;
