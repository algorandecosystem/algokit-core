import type { AccountAssetHolding } from "./index";

/**
 * AccountAssetsInformationResponse contains a list of assets held by an account.
 */
export type AccountAssetsInformation = { round: bigint; nextToken?: string; assetHoldings?: AccountAssetHolding[] };
