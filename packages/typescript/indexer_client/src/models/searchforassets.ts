import type { Asset } from "./index";

/**
 * (empty)
 */
export type SearchForAssets = { assets: Asset[]; currentRound: bigint; nextToken?: string };
