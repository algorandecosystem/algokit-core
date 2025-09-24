/**
 * Fields relating to rewards,
 */
export type BlockRewards = {
  /**
   * \[fees\] accepts transaction fees, it can only spend to the incentive pool.
   */
  feeSink: string

  /**
   * \[rwcalr\] number of leftover MicroAlgos after the distribution of rewards-rate MicroAlgos for every reward unit in the next round.
   */
  rewardsCalculationRound: bigint

  /**
   * \[earn\] How many rewards, in MicroAlgos, have been distributed to each RewardUnit of MicroAlgos since genesis.
   */
  rewardsLevel: bigint

  /**
   * \[rwd\] accepts periodic injections from the fee-sink and continually redistributes them as rewards.
   */
  rewardsPool: string

  /**
   * \[rate\] Number of new MicroAlgos added to the participation stake from rewards at the next round.
   */
  rewardsRate: bigint

  /**
   * \[frac\] Number of leftover MicroAlgos after the distribution of RewardsRate/rewardUnits MicroAlgos for every reward unit in the next round.
   */
  rewardsResidue: bigint
}
