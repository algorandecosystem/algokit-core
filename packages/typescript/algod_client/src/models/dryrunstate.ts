import type { TealValue } from "./index";

/**
 * Stores the TEAL eval step data
 */
export type DryrunState = { line: number; pc: number; stack: TealValue[]; scratch?: TealValue[]; error?: string };
