use common::Book;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

#[test]
fn empty_book() {
    let book = Book::new("ethbtc");
    assert_eq!(book.timestamp, 0);
    assert_eq!(book.bids.len(), 0);
    assert_eq!(book.asks.len(), 0);
    assert_eq!(book.is_consistent(), true);
}

#[test]
fn insert_asks() {
    let mut book = Book::new("ethbtc");
    book.asks.set_str("50", "65.5");
    book.asks.set_str("45.0", "35");
    assert_eq!(book.asks.len(), 2);
    let (price, volume) = book.asks.best().unwrap();
    assert_eq!(price.to_string(), "45.0");
    assert_eq!(volume.to_string(), "35");
    book.asks.set_str("45.0", "0.0");
    assert_eq!(book.asks.len(), 1);
    book.asks.set_str("45.0", "0.0");
    assert_eq!(book.asks.len(), 1);
    let (price, volume) = book.asks.best().unwrap();
    assert_eq!(price.to_string(), "50");
    assert_eq!(volume.to_string(), "65.5");
}

#[test]
fn insert_bids() {
    let mut book = Book::new("adabtc");
    book.bids.set_str("45.0", "65.0");
    book.bids.set_str("50", "35.0");
    assert_eq!(book.bids.len(), 2);
    let (price, volume) = book.bids.best().unwrap();
    assert_eq!(price.to_string(), "50");
    assert_eq!(volume.to_string(), "35.0");
    book.bids.set_str("45.0", "0.0");
    assert_eq!(book.bids.len(), 1);
    book.bids.set_str("45.0", "0.0");
    assert_eq!(book.bids.len(), 1);
    let (price, volume) = book.bids.best().unwrap();
    assert_eq!(price.to_string(), "50");
    assert_eq!(volume.to_string(), "35.0");
}

#[test]
fn inconsistent_book() {
    let mut book = Book::new("adabtc");
    book.bids.set_str("51.2", "100.0");
    book.asks.set_str("49.1", "25.0");
    assert_eq!(book.is_consistent(), false);
}
