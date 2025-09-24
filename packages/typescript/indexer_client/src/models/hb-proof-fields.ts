/**
 * \[hbprf\] HbProof is a signature using HeartbeatAddress's partkey, thereby showing it is online.
 */
export type HbProofFields = {
  /**
   * \[s\] Signature of the heartbeat message.
   */
  hbSig?: string

  /**
   * \[p\] Public key of the heartbeat message.
   */
  hbPk?: string

  /**
   * \[p2\] Key for new-style two-level ephemeral signature.
   */
  hbPk2?: string

  /**
   * \[p1s\] Signature of OneTimeSignatureSubkeyOffsetID(PK, Batch, Offset) under the key PK2.
   */
  hbPk1sig?: string

  /**
   * \[p2s\] Signature of OneTimeSignatureSubkeyBatchID(PK2, Batch) under the master key (OneTimeSignatureVerifier).
   */
  hbPk2sig?: string
}
