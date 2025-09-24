/**
 * Box name and its content.
 */
export type Box = {
  /**
   * The round for which this information is relevant
   */
  round: bigint

  /**
   * \[name\] box name, base64 encoded
   */
  name: string

  /**
   * \[value\] box value, base64 encoded.
   */
  value: string
}
