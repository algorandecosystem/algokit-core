/**
 * BoxReference names a box by its name and the application ID it belongs to.
 */
export type BoxReference = {
  /**
   * Application ID to which the box belongs, or zero if referring to the called application.
   */
  app: bigint

  /**
   * Base64 encoded box name
   */
  name: string
}
