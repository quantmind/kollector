import asyncio
import json
import time
from typing import Any, Sequence, cast

from aiohttp import ClientSession, ClientWebSocketResponse, WSMessage, WSMsgType

from . import config
from .book import Book
from .workers import Worker, Workers


class BadState(Exception):
    pass


class WebsocketReconnect(Exception):
    pass


class Publisher:
    async def publish_books(self, books: dict[str, Book]) -> None:
        """Publish a book to the websocket"""
        pass


class WebsocketGateway(Workers):
    """A Gateway consuming websocket messages."""

    def __init__(self, publisher: Publisher = None, pairs: Sequence[str] = ()) -> None:
        super().__init__()
        self._last_update: float = time.time()
        self._message_received: int = 0
        self._ws_connection: ClientWebSocketResponse | None = None
        self.books: dict[str, Book] = {}
        self.publisher = publisher or Publisher()
        self.pairs: tuple[str, ...] = tuple(pairs)
        self.add_worker(Worker(self._connect_and_listen))
        self.add_worker(Worker(self._publish_books))

    # WebsocketGateway interface

    def ws_url(self) -> str:
        """Return the websocket url"""
        raise NotImplementedError

    def on_ws_connection(self) -> None:
        """Callback when a new websocket is connected"""
        self.logger.warning("new websocket connection with %s", self.ws_url())

    def on_text_message(self, msg: WSMessage) -> None:
        """Handle a text message from websocket

        Subclasses should implement this method innit!.
        """
        self.logger.info("Websocket text message: %s", msg.data)

    def on_error_message(self, msg: WSMessage) -> None:
        """Handle an error message from websocket"""
        self.logger.warning("Websocket error: %s", msg.data)
        raise WebsocketReconnect

    def write_json(self, msg: Any) -> None:
        self.write(json.dumps(msg))

    def write(self, msg: str) -> None:
        self.execute(self._send_str, msg)

    def new_book(self) -> Book:
        return Book()

    # Workers interface

    def status(self) -> dict:
        """Am I doing good? If not raise an error"""
        gap: float = time.time() - self._last_update
        if gap > config.STALE_WEBSOCKET_TIMEOUT:
            raise BadState("Websocket connection is stale for %s seconds" % gap)
        return dict(
            last_update=self._last_update, message_received=self._message_received
        )

    # INTERNALS

    async def _send_str(self, msg: str) -> None:
        if self._ws_connection is not None:
            await self._ws_connection.send_str(msg)
        else:
            self.logger.warning("Websocket connection is closed")

    async def _connect_and_listen(self) -> None:
        """Coroutine for connecting and listening to websocket"""
        while True:
            async with ClientSession() as session:
                async with session.ws_connect(self.ws_url()) as ws_connection:
                    self._ws_connection = ws_connection
                    try:
                        self.on_ws_connection()
                        await self._listen_and_consume_messages()
                    except WebsocketReconnect:
                        pass
                    except ConnectionError as exc:
                        self.logger.warning("Websocket connection error: %s", exc)
                delay = 5
                self.logger.warning("reconnect with websocket in %d seconds", delay)
                self._ws_connection = None
                await asyncio.sleep(delay)

    async def _listen_and_consume_messages(self) -> None:
        """Coroutine for consuming websocket messages"""
        async for msg in cast(ClientWebSocketResponse, self._ws_connection):
            self._last_update = time.time()
            self._message_received += 1
            match msg.type:
                case WSMsgType.TEXT:
                    self.on_text_message(msg)
                case WSMsgType.ERROR:
                    self.on_error_message(msg)
                case WSMsgType.CLOSE:
                    self.logger.warning("Websocket connection closed")
                    raise WebsocketReconnect
                case _:
                    self.logger.warning("unhandled message type: %s", msg.type)
            # release loop to avoid starvation from greedy websocket connections
            await asyncio.sleep(0)

    async def _publish_books(self) -> None:
        while True:
            if len(self.books) == len(self.pairs):
                await self.publisher.publish_books(self.books)
            await asyncio.sleep(config.BOOK_PUBLISH_INTERVAL)
