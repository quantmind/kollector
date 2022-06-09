import random
from typing import Any, AsyncGenerator
from unittest import mock

import numpy as np
import pytest
from aiohttp import ClientSession
from aiohttp.test_utils import TestClient, TestServer
from aiohttp.web import Application
from aioresponses import aioresponses

from pyservice import config, gateway, workers
from pyservice.app import create_app
from pyservice.book import Book

from .ws import websocket


@pytest.fixture
def app() -> Application:
    return create_app(pairs="BTCUSDT")


@pytest.fixture(autouse=True)
def mocks(monkeypatch: Any) -> None:
    monkeypatch.setattr(
        gateway,
        "ClientSession",
        lambda: ClientSession(ws_response_class=mock.MagicMock),
    )
    monkeypatch.setattr(workers, "bail_out", mock.MagicMock)


def mock_binance_responses(r: aioresponses) -> None:
    r.get(
        f"{config.BINANCE_SPOT_URL}/api/v3/depth?symbol=BTCUSDT",
        payload=dict(lastUpdateId=19857594115, bids=[], asks=[]),
        repeat=True,
    )
    r.get(config.BINANCE_SPOT_WS_URL, callback=websocket)


@pytest.fixture
async def app_cli(app: Application) -> AsyncGenerator:
    with aioresponses(passthrough=["http://127.0.0.1"]) as r:
        mock_binance_responses(r)
        client = TestClient(TestServer(app))
        await client.start_server()
        try:
            yield client
        finally:
            await client.close()


@pytest.fixture
def book() -> Book:
    return Book()


@pytest.fixture
def rbook() -> Book:
    return random_book()


def random_book(n_min: int = 30, n_max: int = 50) -> Book:
    book = Book()
    N = random.randint(n_min, n_max)
    prices = [(80 + 40 * random.random()) for _ in range(N)]
    volumes = [(1 + 10 * random.random()) for _ in range(N)]
    mid = np.mean(prices)
    for p, v in zip(prices, volumes):
        if p > mid:
            book.asks.set(p, v)
        else:
            book.bids.set(p, v)
    return book
