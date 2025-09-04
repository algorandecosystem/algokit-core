/**
 * Proof of transaction in a block.
 */
export type TransactionProof = { proof: string; stibhash: string; treedepth: bigint; idx: bigint; hashtype: "sha512_256" | "sha256" };
