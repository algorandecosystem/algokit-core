import type { BuildVersion } from './index'
/**
 * algod version information.
 */
export type Version = {
  build: BuildVersion
  genesisHashB64: Uint8Array
  genesisId: string
  versions: string[]
}
