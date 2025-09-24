/**
 * Participation account data that needs to be checked/acted on by the network.
 */
export type ParticipationUpdates = {
  /**
   * \[partupdrmv\] a list of online accounts that needs to be converted to offline since their participation key expired.
   */
  expiredParticipationAccounts?: string[]

  /**
   * \[partupabs\] a list of online accounts that need to be suspended.
   */
  absentParticipationAccounts?: string[]
}
