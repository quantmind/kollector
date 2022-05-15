//! Level 2 Orderbook implementation
//!
//! A level2 order book is a datastructure which maintains ordered price-volume pairs
use rust_decimal::prelude::*;
use std::collections::BTreeMap;
use std::error::Error;
use std::{cmp, fmt};

pub enum Side {
    Bid,
    Ask,
}

/// One side of a Level 2 Order book
#[derive(Clone, Debug)]
pub struct L2 {
    /// ordered mapping of prices to volumes
    orders: BTreeMap<Decimal, Decimal>,
    desc: bool,
}

#[derive(Debug)]
pub struct InconsistentBook {
    details: String,
    asset: String,
}

/// A `Book` represents a level 2 order book data structure
#[derive(Debug, Clone)]
pub struct Book {
    /// asset name
    pub asset: String,
    /// last timestamp the book was updated
    pub timestamp: usize,
    /// level 2 bid prices & sizes
    pub bids: L2,
    /// level 2 ask prices & sizes
    pub asks: L2,
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

    pub fn update(&mut self, price_size: &[(String, String)]) {}

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

    /// worse price in the orderbook side
    pub fn worse(&self) -> Option<(&Decimal, &Decimal)> {
        if self.desc {
            self.orders.iter().next()
        } else {
            self.orders.iter().next_back()
        }
    }

    pub fn at(&self, n: usize) -> Option<(&Decimal, &Decimal)> {
        if self.desc {
            self.orders.iter().nth_back(n)
        } else {
            self.orders.iter().nth(n)
        }
    }

    pub fn best_price(&self) -> Option<Decimal> {
        match self.best() {
            Some((price, _)) => Some(price.clone()),
            None => None,
        }
    }

    pub fn best_price_f32(&self) -> Option<f32> {
        match self.best() {
            Some((price, _)) => price.to_f32(),
            None => None,
        }
    }

    pub fn worse_price(&self) -> Option<Decimal> {
        match self.worse() {
            Some((price, _)) => Some(price.clone()),
            None => None,
        }
    }

    pub fn worse_price_f32(&self) -> Option<f32> {
        match self.worse() {
            Some((price, _)) => price.to_f32(),
            None => None,
        }
    }

    pub fn price_at(&self, n: usize) -> Option<Decimal> {
        match self.at(n) {
            Some((price, _)) => Some(price.clone()),
            None => None,
        }
    }

    pub fn iter(&self) -> L2Iterator {
        L2Iterator {
            iter: self.orders.iter(),
            desc: self.desc,
        }
    }

    /// Calculate the cumulative volume up to a given depth
    ///
    /// # Arguments
    ///
    /// * `depth` - An integer that specifies the number of price levels to include in the cumulative sum
    /// * `decay` - A float for applying an exponential decay rate to depths, set to 0 for no decay
    pub fn depth_volume(&self, depth: usize, decay: f32) -> f32 {
        let mut volume = 0.0;
        for (i, (_, size)) in self.iter().enumerate() {
            if i >= depth {
                return volume;
            }
            volume += size.to_f32().unwrap() * (-decay * i.to_f32().unwrap()).exp();
        }
        volume
    }

    fn clear(&mut self) {
        self.orders.clear();
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
            timestamp: 0,
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

    pub fn is_empty(&self) -> bool {
        self.bids.is_empty() && self.asks.is_empty()
    }

    pub fn get_max_depth(&self) -> usize {
        cmp::max(self.bids.len(), self.asks.len())
    }

    pub fn get_total_depth(&self) -> usize {
        self.bids.len() + self.asks.len()
    }

    /// Clear the book
    pub fn clear(&mut self) {
        self.bids.clear();
        self.asks.clear();
    }

    pub fn get_side(&self, side: &Side) -> &L2 {
        match side {
            Side::Bid => &self.bids,
            Side::Ask => &self.asks,
        }
    }

    pub fn get_side_mut(&mut self, side: &Side) -> &mut L2 {
        match side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        }
    }

    /// Convert the Book into a Result
    ///
    /// This function is useful for raising errors when the book is not consistent
    ///
    /// # Arguments
    ///
    /// * `asset` - The asset for the book - added to the error message if this is not consistent
    pub fn as_result(&self, asset: &str) -> Result<(), InconsistentBook> {
        match self.is_consistent() {
            true => Ok(()),
            false => Err(InconsistentBook::new("crossed book", asset)),
        }
    }

    /// Calculate the total volume up to `depth` and the book bid/ask imbalance.
    ///
    /// It returns a tuple of (volume, imbalance).
    /// The imbalance is a number between -1 and 1.
    ///
    /// * Imbalance 0 means the book is perfectly balanced
    /// * Imbalance 1 means the book is fully on the bid side (buy pressure)
    /// * Imbalance -1 means the book is fully on the ask side (sell pressure)
    ///
    /// # Arguments
    ///
    /// * `depth` - An integer that specifies the number of price levels to include in the cumulative sum
    /// * `decay` - A float for applying an exponential decay rate to depths, set to 0 for no decay
    pub fn volume_and_imbalance(&self, depth: usize, decay: f32) -> (f32, f32) {
        let volume_bid = self.bids.depth_volume(depth, decay);
        let volume_ask = self.asks.depth_volume(depth, decay);
        let volume = volume_bid + volume_ask;
        match volume {
            0.0 => (0.0, 0.0),
            _ => (volume, (volume_bid - volume_ask) / volume),
        }
    }
}
