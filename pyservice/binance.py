import asyncio
from collections import defaultdict
from typing import Any

from aiohttp import ClientSession, WSMessage

from . import config
from .gateway import WebsocketGateway, Worker


class Binance(WebsocketGateway):
    """Connect to binance websocket and subscribe to depth updates"""

    def __init__(self, **kwargs: Any) -> None:
        super().__init__(**kwargs)
        self._inflight_snapshot: dict[str, bool] = defaultdict(bool)
        self._updates: dict[str, list[dict]] = defaultdict(list)
        self._id: int = 0
        self.add_worker(Worker(self._refresh_snapshots))

    def ws_url(self) -> str:
        return config.BINANCE_SPOT_WS_URL

    def on_ws_connection(self) -> None:
        """New connection, subscribe to depth channels"""
        super().on_ws_connection()
        self.books.clear()
        self._updates.clear()
        self._inflight_snapshot.clear()
        self._rpc_write(
            "SUBSCRIBE", [f"{pair}@depth@100ms".lower() for pair in self.pairs]
        )

    def on_text_message(self, msg: WSMessage) -> None:
        """Handle a text message from websocket"""
        data = msg.json()
        if data.get("e") == "depthUpdate":
            self._on_book_update(data["s"].lower(), data)

    # INTERNALS

    def _rpc_write(self, method: str, params: list) -> None:
        msg = dict(method=method, id=self._rpc_id(), params=params)
        self.logger.info("sending %s", msg)
        self.write_json(msg)

    def _rpc_id(self) -> int:
        self._id += 1
        return self._id

    def _on_book_update(self, pair: str, data: dict) -> None:
        """New depth update"""
        if pair in self.books:
            self._update_book(pair, data)
        else:
            # book snapshot is missing - request one if not done already
            self._request_snapshot(pair)
            self._updates[pair].append(data)

    async def _get_snapshot(self, pair: str) -> None:
        """Request a snapshot of the book for a pair"""
        async with ClientSession() as session:
            self.logger.info("fetch snapshot for %s", pair)
            response = await session.get(
                f"{config.BINANCE_SPOT_URL}/api/v3/depth",
                params=dict(symbol=pair.upper()),
            )
            snapshot = await response.json()
            book = self.new_book()
            sequence = snapshot["lastUpdateId"]
            book.asks.update(snapshot.get("asks", ()))
            book.bids.update(snapshot.get("bids", ()))
            self.books[pair] = book
            self._inflight_snapshot[pair] = False
            for update in self._updates.pop(pair, ()):
                if update["u"] > sequence:
                    # stop updating if book inconsistent
                    if not self._update_book(pair, update):
                        return
            self.logger.info("snapshot for %s populated - ready for updates", pair)

    def _update_book(self, pair: str, update: dict) -> bool:
        book = self.books[pair]
        book.asks.update(update.get("a", ()))
        book.bids.update(update.get("b", ()))
        if not book.is_consistent():
            if not self._inflight_snapshot[pair]:
                self.logger.warning("book is inconsistent - request snapshot")
                self._request_snapshot(pair)
            return False
        return True

    def _request_snapshot(self, pair: str) -> None:
        """Request a snapshot of the book for a pair"""
        if not self._inflight_snapshot[pair]:
            self.books.pop(pair, None)
            self._inflight_snapshot[pair] = True
            self.execute(self._get_snapshot, pair)

    async def _refresh_snapshots(self) -> None:
        """Refresh snapshots for all pairs at regular intervals

        This is so we keep the book consistent for deep levels too
        """
        while True:
            for pair in tuple(self.books):
                self._request_snapshot(pair)
            await asyncio.sleep(config.BINANCE_REFRESH_SNAPSHOT_INTERVAL)
