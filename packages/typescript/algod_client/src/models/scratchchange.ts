import type { AvmValue } from "./index";

/**
 * A write operation into a scratch slot.
 */
export type ScratchChange = { slot: number; "new-value": AvmValue };
