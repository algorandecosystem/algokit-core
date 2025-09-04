/**
 * Describes an asset held by an account.
 *
 * Definition:
 * data/basics/userBalance.go : AssetHolding
 */
export type AssetHolding = { amount: bigint; "asset-id": bigint; "is-frozen": boolean };
