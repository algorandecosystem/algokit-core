import type { Block } from "./index";

/**
 * (empty)
 */
export type SearchForBlockHeaders = { currentRound: bigint; nextToken?: string; blocks: Block[] };
