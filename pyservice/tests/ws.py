import base64
import hashlib
from typing import Any
from unittest.mock import MagicMock

from aiohttp import ClientResponse, hdrs
from aiohttp.http import WS_KEY
from aioresponses import CallbackResult
from yarl import URL


def websocket(url: str, headers: dict, **kwargs: Any) -> CallbackResult:
    sec_key = headers[hdrs.SEC_WEBSOCKET_KEY].encode("utf-8")
    accept = base64.b64encode(hashlib.sha1(sec_key + WS_KEY).digest()).decode()
    headers = {
        hdrs.SEC_WEBSOCKET_ACCEPT: accept,
        hdrs.CONNECTION: "upgrade",
        hdrs.UPGRADE: "websocket",
    }
    return CallbackResult(
        status=101,
        headers=headers,
        response_class=websocket_response_factory,  # type: ignore
    )


def connection_mock() -> MagicMock:
    connection = MagicMock()
    transport = MagicMock()
    transport.is_closing.return_value = False
    connection.transport = transport
    return connection


def websocket_response_factory(method: str, url: URL, **kwargs: Any) -> ClientResponse:
    resp = ClientResponse(method, url, **kwargs)
    resp._connection = connection_mock()
    return resp
