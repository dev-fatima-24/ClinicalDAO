from fastapi import APIRouter, HTTPException
from api.models.schemas import MilestoneRelease
from api.stellar.client import submit_xdr

router = APIRouter(prefix="/milestones", tags=["milestones"])


@router.post("/{proposal_id}/release", status_code=200)
def release_milestone(proposal_id: int, body: MilestoneRelease):
    """
    Trigger a milestone fund release.
    Caller submits a pre-signed Soroban `release_milestone` invocation XDR.
    Off-chain verification (e.g., IRB approval, data submission) is assumed
    complete before this endpoint is called.
    """
    if body.proposal_id != proposal_id:
        raise HTTPException(400, "proposal_id mismatch")

    result = submit_xdr(body.signed_xdr)
    return {
        "proposal_id": proposal_id,
        "milestone_index": body.milestone_index,
        "tx_hash": result.get("hash"),
        "status": result.get("status"),
    }
