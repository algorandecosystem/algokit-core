/**
 * AccountParticipation describes the parameters used by this account in consensus protocol.
 */
export type AccountParticipation = {
  "selection-participation-key": string;
  "vote-first-valid": bigint;
  "vote-key-dilution": bigint;
  "vote-last-valid": bigint;
  "vote-participation-key": string;
  "state-proof-key"?: string;
};
