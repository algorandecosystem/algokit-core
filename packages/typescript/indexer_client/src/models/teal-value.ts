/**
 * Represents a TEAL value.
 */
export type TealValue = {
  /**
   * type of the value. Value `1` refers to **bytes**, value `2` refers to **uint**
   */
  type: bigint

  /**
   * bytes value.
   */
  bytes: string

  /**
   * uint value.
   */
  uint: bigint
}
