/**
 * A health check response.
 */
export type HealthCheck = {
  /**
   * Current version.
   */
  version: string
  data?: {}
  round: bigint
  isMigrating: boolean
  dbAvailable: boolean
  message: string
  errors?: string[]
}
