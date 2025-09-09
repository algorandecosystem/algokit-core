export type StateProofTracking = {
  /**
   * State Proof Type. Note the raw object uses map with this as key.
   */
  type?: bigint;

  /**
   * \[v\] Root of a vector commitment containing online accounts that will help sign the proof.
   */
  votersCommitment?: string;

  /**
   * \[t\] The total number of microalgos held by the online accounts during the StateProof round.
   */
  onlineTotalWeight?: bigint;

  /**
   * \[n\] Next round for which we will accept a state proof transaction.
   */
  nextRound?: bigint;
};
