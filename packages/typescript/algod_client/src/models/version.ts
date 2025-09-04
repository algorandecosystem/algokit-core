import type { BuildVersion } from "./index";

/**
 * algod version information.
 */
export type Version = { build: BuildVersion; genesis_hash_b64: string; genesis_id: string; versions: string[] };
