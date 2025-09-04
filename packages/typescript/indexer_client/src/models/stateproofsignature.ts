import type { MerkleArrayProof } from "./index";

export type StateProofSignature = {
  "falcon-signature"?: string;
  "merkle-array-index"?: number;
  proof?: MerkleArrayProof;
  "verifying-key"?: string;
};
