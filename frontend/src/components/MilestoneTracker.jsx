import { useState, useEffect } from "react";
import { useEscrow } from "../hooks/useEscrow";

const MILESTONE_LABELS = ["Pending", "Approved", "Released"];

export default function MilestoneTracker({ publicKey, sign, proposalId }) {
  const [escrow, setEscrow] = useState(null);
  const { releaseMilestone } = useEscrow(publicKey, sign);

  useEffect(() => {
    if (!proposalId) return;
    fetch(`/api/proposals/${proposalId}`)
      .then((r) => r.json())
      .then(setEscrow);
  }, [proposalId]);

  if (!escrow) return <p>Select a proposal to track milestones.</p>;

  const milestones = escrow.milestones || [];
  const released = milestones.filter((m) => m.status === "Released").length;

  return (
    <div>
      <h2>Milestone Tracker — Proposal #{proposalId}</h2>
      <progress value={released} max={milestones.length} />
      <span> {released}/{milestones.length} released</span>
      <ul>
        {milestones.map((m, i) => (
          <li key={i}>
            Milestone {i + 1}: {m.amount} stroops — <em>{m.status}</em>
            {publicKey && m.status === "Approved" && (
              <button onClick={() => releaseMilestone(proposalId, i)}>Release Funds</button>
            )}
          </li>
        ))}
      </ul>
    </div>
  );
}
