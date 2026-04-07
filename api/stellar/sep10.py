"""
SEP-10 verification helper.
The frontend performs the full SEP-10 challenge/response flow and sends
the resulting JWT. This module validates the JWT and extracts the account.
"""
import os
from jose import jwt, JWTError

SEP10_SIGNING_KEY = os.getenv("SEP10_SIGNING_KEY", "")
SEP10_WEB_AUTH_DOMAIN = os.getenv("SEP10_WEB_AUTH_DOMAIN", "clinicaldao.local")


def verify_sep10_jwt(token: str) -> str:
    """Validate SEP-10 JWT and return the verified Stellar account address."""
    try:
        payload = jwt.decode(
            token,
            SEP10_SIGNING_KEY,
            algorithms=["HS256"],
            options={"verify_aud": False},
        )
        account = payload.get("sub", "")
        if not account.startswith("G"):
            raise ValueError("invalid account in JWT")
        return account
    except JWTError as e:
        raise ValueError(f"invalid SEP-10 token: {e}")
