/**
 * Represents a TEAL value.
 */
export type TealValue = {
  /**
   * \[tt\] value type. Value `1` refers to **bytes**, value `2` refers to **uint**
   */
  type: number

  /**
   * \[tb\] bytes value.
   */
  bytes: Uint8Array

  /**
   * \[ui\] uint value.
   */
  uint: bigint
}
