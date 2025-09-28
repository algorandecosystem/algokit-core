"""
AlgoKit Algod Client Library Python Bindings
"""

# Import all symbols from the Rust extension module and re-export them
from .algod_client import *

# Add any additional exports or initialization here
from .algokit_http_client import HttpClient, HttpMethod, HttpResponse
import typing
import requests

from .algod_client import AlgodClient as FfiAlgodClient

class DefaultHttpClient(HttpClient):
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

class AlgodClient(FfiAlgodClient):
    def __init__(self, *, host: str, token: str, default_headers: typing.Optional[dict[str, str]] = {}):
        super().__init__(DefaultHttpClient(host=host, token=token, default_headers=default_headers))
