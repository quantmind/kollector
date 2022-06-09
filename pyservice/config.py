import os

STALE_WEBSOCKET_TIMEOUT = int(os.getenv("STALE_WEBSOCKET_TIMEOUT", "10"))
BOOK_PUBLISH_INTERVAL = float(os.getenv("BOOK_PUBLISH_INTERVAL", "0.5"))
CURRENCY_PAIRS = os.getenv("CURRENCY_PAIRS", "btcusdt,ethusdt")

BINANCE_SPOT_WS_URL = os.getenv(
    "BINANCE_SPOT_WS_URL", "wss://stream.binance.com:9443/ws"
)
BINANCE_SPOT_URL = os.getenv("BINANCE_SPOT_URL", "https://api.binance.com")
BINANCE_REFRESH_SNAPSHOT_INTERVAL = int(
    os.getenv("BINANCE_REFRESH_SNAPSHOT_INTERVAL", "10")
)
