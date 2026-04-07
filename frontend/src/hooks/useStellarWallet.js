import { useState, useCallback } from "react";
import {
  isConnected,
  getPublicKey,
  signTransaction,
} from "@stellar/freighter-api";
import { sep10Login } from "../utils/sep10";

export function useStellarWallet() {
  const [publicKey, setPublicKey] = useState(null);
  const [jwt, setJwt] = useState(null);
  const [error, setError] = useState(null);

  const connect = useCallback(async () => {
    try {
      const connected = await isConnected();
      if (!connected) throw new Error("Freighter not installed or locked");

      const key = await getPublicKey();
      setPublicKey(key);

      const token = await sep10Login(key);
      setJwt(token);
      return { publicKey: key, jwt: token };
    } catch (e) {
      setError(e.message);
      throw e;
    }
  }, []);

  const sign = useCallback(
    async (xdr) => {
      const { signedXDR } = await signTransaction(xdr, {
        accountToSign: publicKey,
      });
      return signedXDR;
    },
    [publicKey]
  );

  return { publicKey, jwt, error, connect, sign };
}
