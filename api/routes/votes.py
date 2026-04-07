from fastapi import APIRouter, HTTPException
from api.models.schemas import VoteCreate
from api.stellar.client import submit_xdr

router = APIRouter(prefix="/votes", tags=["votes"])

_votes: list[dict] = []


@router.get("/")
def list_votes(proposal_id: int | None = None):
    if proposal_id is not None:
        return [v for v in _votes if v["proposal_id"] == proposal_id]
    return _votes


@router.post("/", status_code=201)
def cast_vote(body: VoteCreate):
    """
    Submit a pre-signed vote transaction XDR to the network.
    The frontend builds and signs the Soroban `cast_vote` invocation.
    """
    result = submit_xdr(body.signed_xdr)
    record = {
        "voter_address": body.voter_address,
        "proposal_id": body.proposal_id,
        "support": body.support,
        "tx_hash": result.get("hash"),
    }
    _votes.append(record)
    return record
