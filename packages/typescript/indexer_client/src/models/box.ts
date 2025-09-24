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
  name: Uint8Array

  /**
   * \[value\] box value, base64 encoded.
   */
  value: Uint8Array
}
