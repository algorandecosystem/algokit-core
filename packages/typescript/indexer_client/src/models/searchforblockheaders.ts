import type { Block } from "./index";

/**
 * (empty)
 */
export type SearchForBlockHeaders = { "current-round": number; "next-token"?: string; blocks: Block[] };
