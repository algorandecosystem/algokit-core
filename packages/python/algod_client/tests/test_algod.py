from algod_client import AlgodClient, DefaultHttpClient
import pytest


@pytest.mark.asyncio
async def test_algod_client():
    client = AlgodClient(DefaultHttpClient(host="http://localhost:4001", token="a" * 64))

    status = await client.get_status()

    assert status is not None
    assert status.last_round > 1000
