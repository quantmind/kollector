from aiohttp.test_utils import TestClient


async def test_status(app_cli: TestClient) -> None:
    response = await app_cli.get("/status")
    assert response.status == 200
    data = await response.json()
    assert data["binance"]
