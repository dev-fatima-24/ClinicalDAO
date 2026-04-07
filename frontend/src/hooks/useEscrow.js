import { useCallback } from "react";
import { buildContractInvocation } from "../utils/stellar";
import { Address, nativeToScVal } from "@stellar/stellar-sdk";

const ESCROW_ID = import.meta.env.VITE_ESCROW_CONTRACT_ID;

export function useEscrow(publicKey, sign) {
  const releaseMilestone = useCallback(
    async (proposalId, milestoneIndex) => {
      const xdrTx = await buildContractInvocation(publicKey, ESCROW_ID, "release_milestone", [
        nativeToScVal(proposalId, { type: "u64" }),
        nativeToScVal(milestoneIndex, { type: "u32" }),
      ]);
      const signed = await sign(xdrTx);
      const res = await fetch(`/api/milestones/${proposalId}/release`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ proposal_id: proposalId, milestone_index: milestoneIndex, signed_xdr: signed }),
      });
      return res.json();
    },
    [publicKey, sign]
  );

  return { releaseMilestone };
}
