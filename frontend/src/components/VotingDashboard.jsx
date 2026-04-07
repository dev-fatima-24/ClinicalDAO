import { useState, useEffect } from "react";
import { useGovernance } from "../hooks/useGovernance";

export default function VotingDashboard({ publicKey, sign }) {
  const [proposals, setProposals] = useState([]);
  const { castVote } = useGovernance(publicKey, sign);

  useEffect(() => {
    fetch("/api/proposals?status=active")
      .then((r) => r.json())
      .then(setProposals);
  }, []);

  async function vote(proposalId, support) {
    await castVote(proposalId, support);
    const updated = await fetch("/api/proposals?status=active").then((r) => r.json());
    setProposals(updated);
  }

  return (
    <div>
      <h2>Active Votes</h2>
      {proposals.length === 0 && <p>No active proposals.</p>}
      <ul>
        {proposals.map((p) => (
          <li key={p.id}>
            <strong>{p.title}</strong>
            <span> For: {p.votes_for} | Against: {p.votes_against}</span>
            {publicKey && (
              <>
                <button onClick={() => vote(p.id, true)}>✓ For</button>
                <button onClick={() => vote(p.id, false)}>✗ Against</button>
              </>
            )}
          </li>
        ))}
      </ul>
    </div>
  );
}
