import type { Asset } from "./index";

/**
 * (empty)
 */
export type LookupAccountCreatedAssets = { assets: Asset[]; "current-round": number; "next-token"?: string };
