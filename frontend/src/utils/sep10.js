/**
 * SEP-10 authentication helpers using Freighter wallet.
 * Full flow: fetch challenge → sign with Freighter → exchange for JWT.
 */
const SEP10_AUTH_ENDPOINT = import.meta.env.VITE_SEP10_AUTH_ENDPOINT || "/api/auth";

export async function sep10Login(publicKey) {
  // 1. Fetch challenge transaction from server
  const challengeRes = await fetch(
    `${SEP10_AUTH_ENDPOINT}?account=${publicKey}`
  );
  const { transaction, network_passphrase } = await challengeRes.json();

  // 2. Sign with Freighter
  const { signTransaction } = await import("@stellar/freighter-api");
  const { signedXDR } = await signTransaction(transaction, {
    networkPassphrase: network_passphrase,
  });

  // 3. Exchange signed XDR for JWT
  const tokenRes = await fetch(SEP10_AUTH_ENDPOINT, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ transaction: signedXDR }),
  });
  const { token } = await tokenRes.json();
  return token;
}
