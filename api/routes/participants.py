from fastapi import APIRouter, HTTPException
from api.models.schemas import ParticipantCreate, ParticipantPayout
from api.stellar.client import submit_xdr

router = APIRouter(prefix="/participants", tags=["participants"])

_participants: list[dict] = []
_payouts: list[dict] = []


@router.get("/")
def list_participants(proposal_id: int | None = None):
    if proposal_id is not None:
        return [p for p in _participants if p["proposal_id"] == proposal_id]
    return _participants


@router.post("/", status_code=201)
def register_participant(body: ParticipantCreate):
    record = {
        "proposal_id": body.proposal_id,
        "wallet_address": body.wallet_address,
        "name_hash": body.name_hash,
        "payment_token": body.payment_token,
        "paid": False,
    }
    _participants.append(record)
    return record


@router.post("/payout", status_code=200)
def payout_participant(body: ParticipantPayout):
    """
    Trigger a direct participant payout via the escrow contract.
    Caller submits a pre-signed `pay_participant` invocation XDR.
    """
    result = submit_xdr(body.signed_xdr)

    # Mark participant as paid
    for p in _participants:
        if (
            p["proposal_id"] == body.proposal_id
            and p["wallet_address"] == body.participant_address
        ):
            p["paid"] = True
            break

    payout = {
        "proposal_id": body.proposal_id,
        "participant_address": body.participant_address,
        "amount": body.amount,
        "tx_hash": result.get("hash"),
    }
    _payouts.append(payout)
    return payout


@router.get("/payouts")
def list_payouts(proposal_id: int | None = None):
    if proposal_id is not None:
        return [p for p in _payouts if p["proposal_id"] == proposal_id]
    return _payouts
