import os
import asyncio
from stellar_sdk import Server
from stellar_sdk.call_builder import CallBuilder

HORIZON_URL = os.getenv("HORIZON_URL", "https://horizon-testnet.stellar.org")
GOVERNANCE_CONTRACT_ID = os.getenv("GOVERNANCE_CONTRACT_ID", "")
ESCROW_CONTRACT_ID = os.getenv("ESCROW_CONTRACT_ID", "")


async def listen_contract_events(contract_id: str, handler):
    """Stream Soroban contract events via Horizon SSE."""
    server = Server(HORIZON_URL)
    # Horizon event streaming for Soroban contracts (testnet)
    endpoint = (
        f"{HORIZON_URL}/contracts/{contract_id}/events?cursor=now"
    )
    import httpx
    async with httpx.AsyncClient(timeout=None) as client:
        async with client.stream("GET", endpoint) as response:
            async for line in response.aiter_lines():
                if line.startswith("data:"):
                    import json
                    data = json.loads(line[5:])
                    await handler(data)
