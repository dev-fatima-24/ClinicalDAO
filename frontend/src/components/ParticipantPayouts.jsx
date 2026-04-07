import { useState, useEffect } from "react";

export default function ParticipantPayouts({ proposalId }) {
  const [payouts, setPayouts] = useState([]);

  useEffect(() => {
    const url = proposalId
      ? `/api/participants/payouts?proposal_id=${proposalId}`
      : "/api/participants/payouts";
    fetch(url)
      .then((r) => r.json())
      .then(setPayouts);
  }, [proposalId]);

  return (
    <div>
      <h2>Participant Payouts</h2>
      {payouts.length === 0 && <p>No payouts recorded yet.</p>}
      <table>
        <thead>
          <tr>
            <th>Participant</th>
            <th>Amount</th>
            <th>Tx Hash</th>
          </tr>
        </thead>
        <tbody>
          {payouts.map((p, i) => (
            <tr key={i}>
              <td>{p.participant_address}</td>
              <td>{p.amount} stroops</td>
              <td>
                <a
                  href={`https://stellar.expert/explorer/testnet/tx/${p.tx_hash}`}
                  target="_blank"
                  rel="noreferrer"
                >
                  {p.tx_hash?.slice(0, 12)}…
                </a>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
