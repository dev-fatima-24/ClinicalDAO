import os
from stellar_sdk import Server, Network, Keypair, TransactionBuilder, Asset
from stellar_sdk.soroban_rpc import SorobanServer
from stellar_sdk.exceptions import NotFoundError

HORIZON_URL = os.getenv("HORIZON_URL", "https://horizon-testnet.stellar.org")
SOROBAN_RPC_URL = os.getenv("SOROBAN_RPC_URL", "https://soroban-testnet.stellar.org")
NETWORK_PASSPHRASE = (
    Network.TESTNET_NETWORK_PASSPHRASE
    if os.getenv("STELLAR_NETWORK", "testnet") == "testnet"
    else Network.PUBLIC_NETWORK_PASSPHRASE
)

horizon = Server(HORIZON_URL)
soroban = SorobanServer(SOROBAN_RPC_URL)


def submit_xdr(signed_xdr: str) -> dict:
    """Submit a pre-signed transaction XDR to the network."""
    from stellar_sdk import Transaction
    response = soroban.send_transaction(signed_xdr)
    return {"hash": response.hash, "status": response.status}


def get_account_sequence(address: str) -> int:
    account = horizon.accounts().account_id(address).call()
    return int(account["sequence"])


def get_xlm_balance(address: str) -> str:
    try:
        account = horizon.accounts().account_id(address).call()
        for b in account["balances"]:
            if b["asset_type"] == "native":
                return b["balance"]
    except NotFoundError:
        return "0"
    return "0"


def get_token_balance(address: str, asset_code: str, asset_issuer: str) -> str:
    try:
        account = horizon.accounts().account_id(address).call()
        for b in account["balances"]:
            if b.get("asset_code") == asset_code and b.get("asset_issuer") == asset_issuer:
                return b["balance"]
    except NotFoundError:
        return "0"
    return "0"
