import type { AvmKeyValue } from "./index";

/**
 * An application's global/local/box state.
 */
export type ApplicationKvstorage = { kvs: AvmKeyValue[]; account?: string };
