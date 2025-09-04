export type IndexerStateProofMessage = {
  "block-headers-commitment"?: string;
  "voters-commitment"?: string;
  "ln-proven-weight"?: bigint;
  "first-attested-round"?: bigint;
  "latest-attested-round"?: bigint;
};
