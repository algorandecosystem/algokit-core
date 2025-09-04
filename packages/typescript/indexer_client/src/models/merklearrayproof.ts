import type { HashFactory } from "./index";

export type MerkleArrayProof = { path?: string[]; "hash-factory"?: HashFactory; "tree-depth"?: number };
