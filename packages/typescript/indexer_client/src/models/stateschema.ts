/**
 * Represents a \[apls\] local-state or \[apgs\] global-state schema. These schemas determine how much storage may be used in a local-state or global-state for an application. The more space used, the larger minimum balance must be maintained in the account holding the data.
 */
export type StateSchema = { numUint: bigint; numByteSlice: bigint };
