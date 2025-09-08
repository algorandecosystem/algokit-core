import type { HbProofFields } from "./index";

/**
 * Fields for a heartbeat transaction.
 *
 * Definition:
 * data/transactions/heartbeat.go : HeartbeatTxnFields
 */
export type TransactionHeartbeat = { hbAddress: string; hbProof: HbProofFields; hbSeed: string; hbVoteId: string; hbKeyDilution: bigint };
