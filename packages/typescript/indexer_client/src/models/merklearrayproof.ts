import type { HashFactory } from "./index";

export type MerkleArrayProof = { path?: string[]; hashFactory?: HashFactory; treeDepth?: bigint };
