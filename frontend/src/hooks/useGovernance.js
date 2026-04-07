import { useCallback } from "react";
import { buildContractInvocation } from "../utils/stellar";
import { Address, nativeToScVal, xdr } from "@stellar/stellar-sdk";

const GOVERNANCE_ID = import.meta.env.VITE_GOVERNANCE_CONTRACT_ID;

export function useGovernance(publicKey, sign) {
  const submitProposal = useCallback(
    async ({ title, ipfsCid, fundingAmount, milestoneAmounts, votingPeriod, thresholdN, thresholdD }) => {
      const xdrTx = await buildContractInvocation(publicKey, GOVERNANCE_ID, "submit_proposal", [
        new Address(publicKey).toScVal(),
        nativeToScVal(title, { type: "string" }),
        nativeToScVal(ipfsCid, { type: "string" }),
        nativeToScVal(fundingAmount, { type: "i128" }),
        nativeToScVal(milestoneAmounts.length, { type: "u32" }),
        nativeToScVal(votingPeriod, { type: "u64" }),
        nativeToScVal(thresholdN, { type: "u32" }),
        nativeToScVal(thresholdD, { type: "u32" }),
      ]);
      const signed = await sign(xdrTx);
      const res = await fetch("/api/proposals", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${localStorage.getItem("sep10_jwt")}`,
        },
        body: JSON.stringify({ signed_xdr: signed }),
      });
      return res.json();
    },
    [publicKey, sign]
  );

  const castVote = useCallback(
    async (proposalId, support) => {
      const xdrTx = await buildContractInvocation(publicKey, GOVERNANCE_ID, "cast_vote", [
        new Address(publicKey).toScVal(),
        nativeToScVal(proposalId, { type: "u64" }),
        nativeToScVal(support, { type: "bool" }),
      ]);
      const signed = await sign(xdrTx);
      const res = await fetch("/api/votes", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ voter_address: publicKey, proposal_id: proposalId, support, signed_xdr: signed }),
      });
      return res.json();
    },
    [publicKey, sign]
  );

  return { submitProposal, castVote };
}
