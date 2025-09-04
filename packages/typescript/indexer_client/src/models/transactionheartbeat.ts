import type { HbProofFields } from "./index";

/**
 * Fields for a heartbeat transaction.
 *
 * Definition:
 * data/transactions/heartbeat.go : HeartbeatTxnFields
 */
export type TransactionHeartbeat = {
  "hb-address": string;
  "hb-proof": HbProofFields;
  "hb-seed": string;
  "hb-vote-id": string;
  "hb-key-dilution": bigint;
};
