import type { AssetParams } from "./index";

/**
 * Specifies both the unique identifier and the parameters for an asset
 */
export type Asset = { index: bigint; params: AssetParams };
