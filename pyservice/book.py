from decimal import Decimal
from math import exp
from operator import neg
from typing import Any, Collection, Iterable, Iterator, NamedTuple

from sortedcontainers import SortedDict

BID = 0
ASK = 1
ZERO = Decimal(0)
HALF = Decimal("0.5")
NAN = Decimal("nan")
NumberOrString = float | Decimal | str


def as_decimal(value: NumberOrString) -> Decimal:
    if isinstance(value, float):
        return Decimal(str(value))
    elif isinstance(value, Decimal):
        return value
    else:
        return Decimal(value)


class SideHelper(NamedTuple):
    side: int
    sign: int
    name: str


class L2(SortedDict):
    @property
    def best(self) -> tuple[Decimal, Decimal]:
        try:
            return self.peekitem(0)
        except IndexError:
            return (NAN, ZERO)

    @property
    def best_price(self) -> Decimal:
        return self.best[0]

    def update(self, data: Iterable[tuple[NumberOrString, NumberOrString]]) -> None:
        for price, volume in data:
            self.set(price, volume)

    def set(
        self, price: NumberOrString, volume: NumberOrString
    ) -> tuple[Decimal, Decimal]:
        price = as_decimal(price)
        volume = as_decimal(volume)
        if volume == ZERO:
            self.pop(price, None)
        else:
            self[price] = volume
        return (price, volume)

    def depth_decay(self, level: int = 3, decay: float = 0) -> float:
        return sum(self._first(level, decay=decay))

    def _first(self, level: int, decay: float = 0) -> Iterator[float]:
        for idx, key in enumerate(self):
            if idx >= level:
                break
            d = exp(-decay * idx)
            yield d * float(self[key])


class Book(Collection[Decimal]):
    """Level 2 order book"""

    def __init__(self, timestamp: int = 0, max_depth: int = 0) -> None:
        self.timestamp: int = timestamp
        self.sides: dict[int, L2] = {BID: L2(neg), ASK: L2()}
        self.max_depth: int = max_depth

    def __repr__(self) -> str:
        return f"bids: {repr(self.bids)}, asks: {repr(self.asks)}"

    def __len__(self) -> int:
        return len(self.bids) + len(self.asks)

    def __iter__(self) -> Iterator[Decimal]:
        yield from self.asks
        yield from self.bids

    def __contains__(self, price: Any) -> bool:
        return price in self.asks or price in self.bids

    @property
    def asks(self) -> L2:
        return self.sides[ASK]

    @property
    def bids(self) -> L2:
        return self.sides[BID]

    def best_bid(self) -> Decimal:
        return self.bids.best_price

    def best_ask(self) -> Decimal:
        return self.asks.best_price

    def best_volume_bid(self) -> Decimal:
        return self.bids.best_volume

    def best_volume_ask(self) -> Decimal:
        return self.asks.best_volume

    def spread(self) -> Decimal:
        return self.best_ask() - self.best_bid()

    def mid(self, weighted: bool = False) -> Decimal:
        bid = self.best_bid()
        ask = self.best_ask()
        if bid == bid and ask == ask:
            if weighted:
                volume_bid = self.best_volume_bid()
                volume_ask = self.best_volume_ask()
                return (volume_bid * bid + volume_ask * ask) / (volume_bid + volume_ask)
            return HALF * (bid + ask)
        elif bid == bid:
            return bid
        else:
            return ask

    def imbalance(self, depth: int = 3, decay: float = 0.5) -> float:
        volume_bid = self.bids.depth_decay(depth, decay)
        volume_ask = self.asks.depth_decay(depth, decay)
        volume = volume_bid + volume_ask
        return 0 if volume == 0 else (volume_bid - volume_ask) / volume

    def is_consistent(self) -> bool:
        """Check if the book is consistent"""
        bid = self.best_bid()
        ask = self.best_ask()
        if bid == bid and ask == ask:
            return bid < ask
        else:
            return True
