import type { ApplicationStateSchema, TealKeyValueStore } from "./index";

/**
 * Stores local state associated with an application.
 */
export type ApplicationLocalState = { id: bigint; schema: ApplicationStateSchema; "key-value"?: TealKeyValueStore };
