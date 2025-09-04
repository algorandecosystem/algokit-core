import type { Asset } from "./index";

/**
 * (empty)
 */
export type SearchForAssets = { assets: Asset[]; "current-round": number; "next-token"?: string };
