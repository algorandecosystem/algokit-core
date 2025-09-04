import type { AssetParams } from "./index";

/**
 * Specifies both the unique identifier and the parameters for an asset
 */
export type Asset = { index: bigint; deleted?: boolean; "created-at-round"?: bigint; "destroyed-at-round"?: bigint; params: AssetParams };
