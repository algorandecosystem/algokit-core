import type { TealValue } from "./index";

/**
 * Stores the TEAL eval step data
 */
export type DryrunState = { line: bigint; pc: bigint; stack: TealValue[]; scratch?: TealValue[]; error?: string };
