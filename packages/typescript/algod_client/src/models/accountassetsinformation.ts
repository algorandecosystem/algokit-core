import type { AccountAssetHolding } from "./index";

/**
 * AccountAssetsInformationResponse contains a list of assets held by an account.
 */
export type AccountAssetsInformation = { round: bigint; "next-token"?: string; "asset-holdings"?: AccountAssetHolding[] };
