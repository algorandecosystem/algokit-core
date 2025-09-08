/**
 * TransactionParams contains the parameters that help a client construct
 * a new transaction.
 */
export type TransactionParams = {
  consensusVersion: string;
  fee: bigint;
  genesisHash: string;
  genesisId: string;
  lastRound: bigint;
  minFee: bigint;
};
