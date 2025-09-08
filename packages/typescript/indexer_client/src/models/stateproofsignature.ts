import type { MerkleArrayProof } from "./index";

export type StateProofSignature = { falconSignature?: string; merkleArrayIndex?: bigint; proof?: MerkleArrayProof; verifyingKey?: string };
