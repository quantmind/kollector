from decimal import Decimal

from pyservice.book import HALF, Book


def test_empty_order_book(book: Book) -> None:
    assert len(book.sides) == 2
    assert len(book.bids) == 0
    assert len(book.asks) == 0
    assert book.best_bid() != book.best_bid()
    assert book.best_ask() != book.best_ask()
    assert book.is_consistent()
    assert book.imbalance() == 0
    assert book.mid() != book.mid()
    p, v = book.bids.best
    assert p != p
    assert v == 0


def test_only_bid(book: Book) -> None:
    assert len(book.sides) == 2
    assert len(book.bids) == 0
    assert len(book.asks) == 0
    book.bids.set(1.2, 10)
    assert len(book.bids) == 1
    assert len(book.asks) == 0
    assert book.is_consistent()
    assert book.imbalance() == 1


def test_only_ask(book: Book) -> None:
    assert len(book.sides) == 2
    assert len(book.bids) == 0
    assert len(book.asks) == 0
    book.asks.set(1.2, 10)
    assert len(book.bids) == 0
    assert len(book.asks) == 1
    assert book.is_consistent()
    assert book.imbalance(1) == -1


def test_order_book(book: Book) -> None:
    book.bids.set(1.1, 10)
    book.asks.set(1.2, 10)
    assert len(book.bids) == 1
    assert len(book.asks) == 1
    assert book.is_consistent()
    assert book.bids.best[1] == Decimal(10)
    assert book.asks.best[1] == Decimal(10)
    assert book.imbalance() == 0
    book.asks.set(1.2, 5)
    assert book.bids.best[1] == Decimal(10)
    assert book.asks.best[1] == Decimal(5)
    assert book.imbalance(1) == float(
        (Decimal(10) - Decimal(5)) / (Decimal(10) + Decimal(5))
    )


def test_ordering(rbook: Book) -> None:
    assert rbook.is_consistent()
    b0 = float("infinity")
    for bid in rbook.bids:
        assert b0 > bid
        b0 = bid
    a0 = 0
    for ask in rbook.asks:
        assert a0 < ask
        a0 = ask


def test_mid(rbook: Book) -> None:
    mid = rbook.mid()
    for bid in rbook.bids:
        assert bid < mid
    for ask in rbook.asks:
        assert ask > mid


def test_container(rbook: Book) -> None:
    assert rbook
    assert len(rbook) == len(rbook.bids) + len(rbook.asks)
    prices = list(rbook)
    assert len(prices) == len(rbook)
    for p in prices:
        assert p in rbook


def test_replace(rbook: Book) -> None:
    assert rbook.is_consistent()
    prices = rbook.bids if len(rbook.bids) > len(rbook.asks) else rbook.asks
    price_list = list(prices.items())
    N = len(price_list)
    #
    h = int(N / 2)
    price = price_list[h][0]
    assert price_list[h][1]
    #
    # add new price
    price = HALF * (price_list[h - 1][0] + price_list[h][0])
    prices[price] = 50
    assert len(prices) == N + 1


def test_remove_level(book: Book) -> None:
    assert book.mid() != book.mid()
    assert book.spread() != book.spread()
    book.bids.set(45, 20)
    assert len(book.bids) == 1
    assert book.mid() == Decimal(45)
    assert book.spread() != book.spread()
    book.bids.set(45, 0)
    assert book.mid() != book.mid()
    assert book.spread() != book.spread()
    assert len(book.bids) == 0
