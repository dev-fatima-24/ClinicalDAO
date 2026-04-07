import { useState } from "react";
import { BrowserRouter, Routes, Route, Link } from "react-router-dom";
import { useStellarWallet } from "./hooks/useStellarWallet";
import ProposalBoard from "./components/ProposalBoard";
import VotingDashboard from "./components/VotingDashboard";
import MilestoneTracker from "./components/MilestoneTracker";
import ParticipantPayouts from "./components/ParticipantPayouts";

export default function App() {
  const { publicKey, jwt, error, connect, sign } = useStellarWallet();
  const [selectedProposal, setSelectedProposal] = useState(null);

  return (
    <BrowserRouter>
      <nav>
        <Link to="/">Proposals</Link> |{" "}
        <Link to="/vote">Vote</Link> |{" "}
        <Link to="/milestones">Milestones</Link> |{" "}
        <Link to="/payouts">Payouts</Link>
        <span style={{ float: "right" }}>
          {publicKey ? (
            <span>🔑 {publicKey.slice(0, 8)}…</span>
          ) : (
            <button onClick={connect}>Connect Wallet</button>
          )}
          {error && <span style={{ color: "red" }}> {error}</span>}
        </span>
      </nav>

      <Routes>
        <Route path="/" element={<ProposalBoard publicKey={publicKey} sign={sign} jwt={jwt} />} />
        <Route path="/vote" element={<VotingDashboard publicKey={publicKey} sign={sign} />} />
        <Route
          path="/milestones"
          element={
            <MilestoneTracker
              publicKey={publicKey}
              sign={sign}
              proposalId={selectedProposal}
            />
          }
        />
        <Route path="/payouts" element={<ParticipantPayouts proposalId={selectedProposal} />} />
      </Routes>
    </BrowserRouter>
  );
}
