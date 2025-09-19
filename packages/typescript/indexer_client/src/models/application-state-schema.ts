/**
 * Specifies maximums on the number of each type that may be stored.
 */
export type ApplicationStateSchema = {
  /**
   * number of uints.
   */
  numUint: bigint;

  /**
   * number of byte slices.
   */
  numByteSlice: bigint;
};
