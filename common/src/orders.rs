//! Level 2 Orderbook implementation
//!
//! A level2 order book is a datastructure which maintains ordered price-volume pairs
use rust_decimal::prelude::*;
use std::collections::BTreeMap;
use std::error::Error;
use std::{cmp, fmt};

type L2Map = BTreeMap<Decimal, Decimal>;

/// Orderbook side
pub enum Side {
    Bid,
    Ask,
}

/// One side of a Level 2 Order book
#[derive(Clone, Debug)]
pub struct L2 {
    /// ordered mapping of prices to volumes
    orders: L2Map,
    desc: bool,
}

#[derive(Debug)]
pub struct InconsistentBook {
    pub details: String,
    pub asset: String,
}

/// A `Book` represents a level 2 order book data structure
#[derive(Debug, Clone)]
pub struct Book {
    /// asset name
    pub asset: String,
    /// level 2 bid prices & sizes
    pub bids: L2,
    /// level 2 ask prices & sizes
    pub asks: L2,
}

/// Calculate ask-bid spread
pub fn bid_ask_spread(bid: Option<Decimal>, ask: Option<Decimal>) -> Option<Decimal> {
    match bid {
        Some(b) => ask.map(|a| a - b),
        None => None,
    }
}

pub struct L2Iterator<'a> {
    iter: std::collections::btree_map::Iter<'a, Decimal, Decimal>,
    desc: bool,
}

impl L2 {
    fn new(desc: bool) -> L2 {
        L2 {
            orders: BTreeMap::new(),
            desc,
        }
    }

    /// Returns the depth levels
    pub fn len(&self) -> usize {
        self.orders.len()
    }

    /// Returns true if this orderbook side is empty
    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    /// Set a new price/volume into the book side
    ///
    /// # Arguments
    ///
    /// * `price` - the price level
    /// * `volume` - volume fro the price, if 0 the price level will be removed
    pub fn set(&mut self, price: Decimal, volume: Decimal) {
        match volume.is_zero() {
            true => self.orders.remove(&price),
            false => self.orders.insert(price, volume),
        };
    }

    /// Set a new price/volume into the book side
    ///
    /// # Arguments
    ///
    /// * `price` - the price level
    /// * `volume` - volume fro the price, if 0 the price level will be removed
    pub fn set_str(&mut self, price: &str, volume: &str) {
        self.set(
            Decimal::from_str(price).unwrap(),
            Decimal::from_str(volume).unwrap(),
        )
    }

    /// Update the order side with a vector of price/volume tuples
    pub fn update(&mut self, price_volume: &[(String, String)]) {
        for (price, volume) in price_volume.iter() {
            self.set_str(price, volume);
        }
    }

    /// Returns the (price, volume) tuple at the best price if available
    pub fn best(&self) -> Option<(&Decimal, &Decimal)> {
        if self.desc {
            self.orders.iter().next_back()
            // use this once available in stable
            //self.orders.last_key_value()
        } else {
            self.orders.iter().next()
            // self.orders.first_key_value()
        }
    }

    /// Best price in the l2 side
    pub fn best_price(&self) -> Option<Decimal> {
        self.best().map(|(price, _)| *price)
    }

    /// Best of price
    ///
    /// This function returns the best price between the price provided and
    /// the current best price in the l2 side
    pub fn best_of(&self, price: Option<Decimal>) -> Option<Decimal> {
        match self.best_price() {
            Some(best) => match price {
                Some(other_price) => match self.desc {
                    true => Some(cmp::max(best, other_price)),
                    false => Some(cmp::min(best, other_price)),
                },
                None => Some(best),
            },
            None => price,
        }
    }

    /// (price, volume) tuple Iterator
    pub fn iter(&self) -> L2Iterator {
        L2Iterator {
            iter: self.orders.iter(),
            desc: self.desc,
        }
    }

    fn trim(&self, max_depth: usize) -> Self {
        let mut orders = L2Map::new();
        for (i, (price, volume)) in self.iter().enumerate() {
            if i >= max_depth {
                break;
            }
            orders.insert(*price, *volume);
        }
        Self {
            orders,
            desc: self.desc,
        }
    }
}

impl<'a> Iterator for L2Iterator<'a> {
    type Item = (&'a Decimal, &'a Decimal);

    fn next(&mut self) -> Option<Self::Item> {
        match self.desc {
            false => self.iter.next(),
            true => self.iter.next_back(),
        }
    }
}

impl InconsistentBook {
    pub fn new(msg: &str, asset: &str) -> Self {
        InconsistentBook {
            details: msg.to_owned(),
            asset: asset.to_owned(),
        }
    }
}

impl fmt::Display for InconsistentBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for InconsistentBook {
    fn description(&self) -> &str {
        &self.details
    }
}

impl Book {
    pub fn new(asset: &str) -> Self {
        Self {
            asset: asset.to_owned(),
            bids: L2::new(true),
            asks: L2::new(false),
        }
    }

    /// Check if the book is consistent
    ///
    /// A consistent book has the best bid price lower than the best ask price.
    /// In other words no crossing allowed
    pub fn is_consistent(&self) -> bool {
        let bid = match self.bids.best() {
            None => return true,
            Some((price, _)) => price,
        };
        let ask = match self.asks.best() {
            None => return true,
            Some((price, _)) => price,
        };
        bid < ask
    }

    pub fn spread(&self) -> Option<Decimal> {
        bid_ask_spread(self.bids.best_price(), self.asks.best_price())
    }

    /// Return a new book trimmed a max depth
    pub fn trim(&self, max_depth: usize) -> Self {
        Self {
            asset: self.asset.to_owned(),
            asks: self.asks.trim(max_depth),
            bids: self.bids.trim(max_depth),
        }
    }
}
