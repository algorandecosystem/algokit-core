/**
 * TransactionParams contains the parameters that help a client construct
 * a new transaction.
 */
export type TransactionParams = {
  "consensus-version": string;
  fee: bigint;
  "genesis-hash": string;
  "genesis-id": string;
  "last-round": bigint;
  "min-fee": bigint;
};
