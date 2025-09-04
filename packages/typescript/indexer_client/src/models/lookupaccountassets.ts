import type { AssetHolding } from "./index";

/**
 * (empty)
 */
export type LookupAccountAssets = { "current-round": number; "next-token"?: string; assets: AssetHolding[] };
