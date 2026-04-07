from pydantic import BaseModel, Field
from typing import Optional
from enum import Enum


class ProposalStatus(str, Enum):
    active = "active"
    passed = "passed"
    rejected = "rejected"
    executed = "executed"


class ProposalCreate(BaseModel):
    researcher_address: str
    title: str
    ipfs_cid: str
    funding_amount: int
    milestone_amounts: list[int]
    voting_period_secs: int = 604800  # 1 week default
    threshold_numerator: int = 2
    threshold_denominator: int = 3


class ProposalResponse(BaseModel):
    id: int
    researcher_address: str
    title: str
    ipfs_cid: str
    funding_amount: int
    votes_for: int
    votes_against: int
    status: ProposalStatus
    deadline: int
    contract_proposal_id: Optional[int] = None


class VoteCreate(BaseModel):
    voter_address: str
    proposal_id: int
    support: bool
    signed_xdr: str  # pre-signed transaction XDR from frontend


class MilestoneRelease(BaseModel):
    proposal_id: int
    milestone_index: int
    signed_xdr: str


class ParticipantCreate(BaseModel):
    proposal_id: int
    wallet_address: str
    name_hash: str  # hashed — no PII stored
    payment_token: str = "XLM"  # "XLM" or USDC contract address


class ParticipantPayout(BaseModel):
    proposal_id: int
    participant_address: str
    amount: int
    signed_xdr: str
