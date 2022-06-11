import logging

import click
from aiohttp import web

from . import config
from .binance import Binance
from .console import ConsoleUI

routes = web.RouteTableDef()


@routes.get("/status")
async def status(request: web.Request) -> web.Response:
    """status probe"""
    status = {gateway.name: gateway.status() for gateway in request.app["gateways"]}
    return web.json_response(status)


def create_app(
    console: bool = False, pairs: str = "", close_ws_every: int = 0
) -> web.Application:
    """Create aiohttp web application and register gateways"""
    app = web.Application()
    publisher = ConsoleUI.setup(app) if console else None
    pairs_ = [
        pair.strip().lower() for pair in (pairs or config.CURRENCY_PAIRS).split(",")
    ]
    app["gateways"] = [
        Binance.setup(
            app, publisher=publisher, pairs=pairs_, close_ws_every=close_ws_every
        )
    ]
    app.router.add_routes(routes)
    return app


@click.command()
@click.option("--console", default=False, is_flag=True, help="add terminal console")
@click.option(
    "--pairs",
    default=config.CURRENCY_PAIRS,
    help="comma separated list of currency pairs to subscribe",
    show_default=True,
)
@click.option(
    "--drop",
    default=0,
    type=int,
    help="number of seconds after which the websocket connection is dropped",
    show_default=True,
)
@click.option(
    "--port", default=3010, type=int, help="port to listen on", show_default=True
)
def start_app(console: bool, pairs: str, drop: int, port: int) -> None:
    """Start the service"""
    if not console:
        logging.basicConfig(level=logging.INFO)
    app = create_app(console=console, pairs=pairs, close_ws_every=drop)
    web.run_app(app, port=port)
