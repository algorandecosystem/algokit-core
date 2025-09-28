import typing
from algod_client import AlgodClient
from algod_client.algokit_http_client import HttpClient, HttpMethod, HttpResponse
import requests
import pytest

class HttpClientImpl(HttpClient):
    def __init__(self, *, host: str, token: str, default_headers: typing.Optional[dict[str, str]] = {}):
        self.host = host
        self.token = token
        self.default_headers = default_headers or {}
        super().__init__()

    @typing.override
    async def request(  # type: ignore
        self,
        method: HttpMethod,
        path: str,
        query: typing.Optional[dict[str, str]],
        body: typing.Optional[bytes],
        headers: typing.Optional[dict[str, str]],
    ) -> HttpResponse:
        headers = (headers or {}) | self.default_headers
        headers["X-Algo-API-Token"] = self.token

        if method == HttpMethod.GET:
            res = requests.get(f"{self.host}/{path}", params=query, headers=headers)
        elif method == HttpMethod.POST:
            res = requests.post(f"{self.host}/{path}", params=query, data=body, headers=headers)
        else:
            raise NotImplementedError(f"HTTP method {method} not implemented in test client")

        if res.status_code != 200:
            raise Exception(f"HTTP request failed: {res.status_code} {res.text}")

        # NOTE: Headers needing to be lowercase was a bit surprising, so we need to make sure we document that
        headers = {k.lower(): v for k, v in res.headers.items()}

        return HttpResponse(
            body=res.content,
            headers=headers
        )

@pytest.mark.asyncio
async def test_algod_client():
    client = AlgodClient(HttpClientImpl(host="http://localhost:4001", token="a" * 64))

    status = await client.get_status()

    assert status is not None
    assert status.last_round > 1000
