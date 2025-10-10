export type TokenHeader = string | Record<string, string>

/** Represents the different Algorand networks */
export enum AlgorandNetwork {
  /** Local development network */
  LocalNet = 'localnet',
  /** Algorand TestNet */
  TestNet = 'testnet',
  /** Algorand MainNet */
  MainNet = 'mainnet',
}

/** Represents the different Algorand services */
export enum AlgorandService {
  /** Algorand daemon (algod) - provides access to the blockchain */
  Algod = 'algod',
  /** Algorand indexer - provides historical blockchain data */
  Indexer = 'indexer',
  /** Key Management Daemon (kmd) - provides key management functionality */
  Kmd = 'kmd',
}

/** Config for an Algorand SDK client. */
export interface AlgoClientConfig {
  /** Base URL of the server e.g. http://localhost, https://testnet-api.algonode.cloud/, etc. */
  server: string
  /** Optional port to use e.g. 4001, 443, etc. */
  port?: number | string
  /** Optional token or headers to use for API authentication */
  token?: TokenHeader
}

/** Configuration for algod, indexer and kmd clients. */
export interface AlgoConfig {
  /** Algod client configuration */
  algodConfig: AlgoClientConfig
  /** Indexer client configuration */
  indexerConfig?: AlgoClientConfig
  /** Kmd client configuration */
  kmdConfig?: AlgoClientConfig
}

/** Details about the currently connected network. */
export interface NetworkDetails {
  /** Whether the network is TestNet */
  isTestnet: boolean
  /** Whether the network is MainNet */
  isMainnet: boolean
  /** Whether the network is a LocalNet */
  isLocalnet: boolean
  /** Genesis ID reported by the network */
  genesisId: string
  /** Genesis hash reported by the network encoded as base64 */
  genesisHash: string
}

/**
 * Returns true if the given network genesisId is associated with a LocalNet network.
 * @param genesisId The network genesis ID
 * @returns Whether the given genesis ID is associated with a LocalNet network
 */
export function genesisIdIsLocalNet(genesisId: string): boolean {
  return genesisId === 'devnet-v1' || genesisId === 'sandnet-v1' || genesisId === 'dockernet-v1'
}
