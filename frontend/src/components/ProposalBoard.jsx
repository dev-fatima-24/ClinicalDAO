import { useState, useEffect } from "react";
import { useGovernance } from "../hooks/useGovernance";

export default function ProposalBoard({ publicKey, sign, jwt }) {
  const [proposals, setProposals] = useState([]);
  const [form, setForm] = useState({ title: "", ipfsCid: "", fundingAmount: "", milestones: "3" });
  const { submitProposal } = useGovernance(publicKey, sign);

  useEffect(() => {
    fetch("/api/proposals")
      .then((r) => r.json())
      .then(setProposals);
  }, []);

  async function handleSubmit(e) {
    e.preventDefault();
    await submitProposal({
      title: form.title,
      ipfsCid: form.ipfsCid,
      fundingAmount: parseInt(form.fundingAmount),
      milestoneAmounts: Array(parseInt(form.milestones)).fill(
        Math.floor(parseInt(form.fundingAmount) / parseInt(form.milestones))
      ),
      votingPeriod: 604800,
      thresholdN: 2,
      thresholdD: 3,
    });
    const updated = await fetch("/api/proposals").then((r) => r.json());
    setProposals(updated);
  }

  return (
    <div>
      <h2>Proposals</h2>

      {publicKey && (
        <form onSubmit={handleSubmit}>
          <input placeholder="Title" value={form.title} onChange={(e) => setForm({ ...form, title: e.target.value })} required />
          <input placeholder="IPFS CID" value={form.ipfsCid} onChange={(e) => setForm({ ...form, ipfsCid: e.target.value })} required />
          <input type="number" placeholder="Funding (stroops)" value={form.fundingAmount} onChange={(e) => setForm({ ...form, fundingAmount: e.target.value })} required />
          <input type="number" placeholder="Milestones" value={form.milestones} onChange={(e) => setForm({ ...form, milestones: e.target.value })} min="1" />
          <button type="submit">Submit Proposal</button>
        </form>
      )}

      <ul>
        {proposals.map((p) => (
          <li key={p.id}>
            <strong>{p.title}</strong> — {p.status} | For: {p.votes_for} | Against: {p.votes_against}
          </li>
        ))}
      </ul>
    </div>
  );
}
