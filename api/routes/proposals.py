from fastapi import APIRouter, Header, HTTPException
from api.models.schemas import ProposalCreate, ProposalResponse
from api.stellar.sep10 import verify_sep10_jwt
from api.stellar.client import submit_xdr
import os

router = APIRouter(prefix="/proposals", tags=["proposals"])

GOVERNANCE_CONTRACT_ID = os.getenv("GOVERNANCE_CONTRACT_ID", "")

# In-memory store — replace with a DB in production
_proposals: dict[int, dict] = {}


@router.get("/", response_model=list[ProposalResponse])
def list_proposals(status: str | None = None):
    items = list(_proposals.values())
    if status:
        items = [p for p in items if p["status"] == status]
    return items


@router.post("/", response_model=ProposalResponse, status_code=201)
def create_proposal(
    body: ProposalCreate,
    authorization: str = Header(...),
):
    """
    Researcher submits a proposal.
    Requires SEP-10 JWT in Authorization: Bearer <token>.
    The frontend must pre-sign the Soroban invocation XDR and include it
    in the request body (not shown here — handled client-side via Freighter).
    """
    token = authorization.removeprefix("Bearer ").strip()
    verified_address = verify_sep10_jwt(token)

    if verified_address != body.researcher_address:
        raise HTTPException(403, "JWT address does not match researcher_address")

    # Assign a local ID (on-chain ID comes back after tx confirmation)
    proposal_id = len(_proposals) + 1
    record = {
        "id": proposal_id,
        "researcher_address": body.researcher_address,
        "title": body.title,
        "ipfs_cid": body.ipfs_cid,
        "funding_amount": body.funding_amount,
        "votes_for": 0,
        "votes_against": 0,
        "status": "active",
        "deadline": 0,
        "contract_proposal_id": None,
    }
    _proposals[proposal_id] = record
    return record


@router.get("/{proposal_id}", response_model=ProposalResponse)
def get_proposal(proposal_id: int):
    if proposal_id not in _proposals:
        raise HTTPException(404, "proposal not found")
    return _proposals[proposal_id]
