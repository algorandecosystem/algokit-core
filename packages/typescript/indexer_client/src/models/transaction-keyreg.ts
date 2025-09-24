/**
 * Fields for a keyreg transaction.
 *
 * Definition:
 * data/transactions/keyreg.go : KeyregTxnFields
 */
export type TransactionKeyreg = {
  /**
   * \[nonpart\] Mark the account as participating or non-participating.
   */
  nonParticipation?: boolean

  /**
   * \[selkey\] Public key used with the Verified Random Function (VRF) result during committee selection.
   */
  selectionParticipationKey?: string

  /**
   * \[votefst\] First round this participation key is valid.
   */
  voteFirstValid?: bigint

  /**
   * \[votekd\] Number of subkeys in each batch of participation keys.
   */
  voteKeyDilution?: bigint

  /**
   * \[votelst\] Last round this participation key is valid.
   */
  voteLastValid?: bigint

  /**
   * \[votekey\] Participation public key used in key registration transactions.
   */
  voteParticipationKey?: string

  /**
   * \[sprfkey\] State proof key used in key registration transactions.
   */
  stateProofKey?: string
}
