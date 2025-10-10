import { AlgodClient, ApiError } from '@algorandfoundation/algod-client'
import { IndexerClient } from '@algorandfoundation/indexer-client'
import { KmdClient } from '@algorandfoundation/kmd-client'
import { Buffer } from 'buffer'

import {
  AlgoClientConfig,
  AlgoConfig,
  AlgorandService,
  NetworkDetails,
  TokenHeader,
  genesisIdIsLocalNet,
} from './network-client'

export interface ClientManagerClients {
  algod: AlgodClient
  indexer?: IndexerClient
  kmd?: KmdClient
}

interface NetworkCache {
  value?: NetworkDetails
  promise?: Promise<NetworkDetails>
}

export class ClientManager {
  private readonly algodClient: AlgodClient
  private readonly indexerClient?: IndexerClient
  private readonly kmdClient?: KmdClient
  private readonly networkCache: NetworkCache = {}

  constructor(clientsOrConfig: ClientManagerClients | AlgoConfig) {
    const clients = 'algod' in clientsOrConfig
      ? clientsOrConfig
      : {
          algod: ClientManager.getAlgodClient(clientsOrConfig.algodConfig),
          indexer: clientsOrConfig.indexerConfig
            ? ClientManager.getIndexerClient(clientsOrConfig.indexerConfig)
            : undefined,
          kmd: clientsOrConfig.kmdConfig ? ClientManager.getKmdClient(clientsOrConfig.kmdConfig) : undefined,
        }

    this.algodClient = clients.algod
    this.indexerClient = clients.indexer
    this.kmdClient = clients.kmd
  }

  /** Returns an Algod API client. */
  get algod(): AlgodClient {
    return this.algodClient
  }

  /** Returns an Indexer API client or throws if not configured. */
  get indexer(): IndexerClient {
    if (!this.indexerClient) {
      throw new Error('Attempt to use Indexer client without configuring one')
    }
    return this.indexerClient
  }

  /** Returns an Indexer API client if present. */
  get indexerIfPresent(): IndexerClient | undefined {
    return this.indexerClient
  }

  /** Returns a KMD API client or throws if not configured. */
  get kmd(): KmdClient {
    if (!this.kmdClient) {
      throw new Error('Attempt to use KMD client without configuring one')
    }
    return this.kmdClient
  }

  /** Returns a KMD API client if present. */
  get kmdIfPresent(): KmdClient | undefined {
    return this.kmdClient
  }

  /** Get details about the current network. */
  async network(): Promise<NetworkDetails> {
    if (this.networkCache.value) {
      return this.networkCache.value
    }

    if (!this.networkCache.promise) {
      this.networkCache.promise = this.fetchNetworkDetails().then((details) => {
        this.networkCache.value = details
        this.networkCache.promise = undefined
        return details
      }).catch((error) => {
        this.networkCache.promise = undefined
        throw error
      })
    }

    return this.networkCache.promise
  }

  /** Returns true if the given genesis ID is associated with a LocalNet network. */
  static genesisIdIsLocalNet(genesisId: string): boolean {
    return genesisIdIsLocalNet(genesisId)
  }

  /** Returns true if the current network is LocalNet. */
  async isLocalNet(): Promise<boolean> {
    const network = await this.network()
    return network.isLocalnet
  }

  /** Returns true if the current network is TestNet. */
  async isTestNet(): Promise<boolean> {
    const network = await this.network()
    return network.isTestnet
  }

  /** Returns true if the current network is MainNet. */
  async isMainNet(): Promise<boolean> {
    const network = await this.network()
    return network.isMainnet
  }

  /**
   * Derive configuration from the environment if possible, otherwise default to a localnet configuration.
   */
  static getConfigFromEnvironmentOrLocalNet(): AlgoConfig {
    try {
      const algodConfig = this.getAlgodConfigFromEnvironment()
      const indexerConfig = this.safeGetConfig(this.getIndexerConfigFromEnvironment.bind(this))

      const isPublicNetwork = /mainnet|testnet/.test(algodConfig.server)
      const kmdConfig = isPublicNetwork
        ? undefined
        : this.safeGetConfig(() => this.getKmdConfigFromEnvironment(algodConfig)) ?? {
            server: algodConfig.server,
            port: this.parsePort(process.env.KMD_PORT) ?? this.parsePort(process.env.ALGOD_PORT) ?? 4002,
            token: process.env.KMD_TOKEN ?? process.env.ALGOD_TOKEN,
          }

      return {
        algodConfig,
        indexerConfig,
        kmdConfig,
      }
    } catch (error) {
      return {
        algodConfig: this.getDefaultLocalnetConfig(AlgorandService.Algod),
        indexerConfig: this.getDefaultLocalnetConfig(AlgorandService.Indexer),
        kmdConfig: this.getDefaultLocalnetConfig(AlgorandService.Kmd),
      }
    }
  }

  /**
   * Returns Indexer configuration derived from environment variables.
   * @throws Error if required environment variables are missing.
   */
  static getIndexerConfigFromEnvironment(): AlgoClientConfig {
    const server = process.env.INDEXER_SERVER
    if (!server) {
      throw new Error('INDEXER_SERVER environment variable not found')
    }

    const port = this.parsePort(process.env.INDEXER_PORT)
    const token = process.env.INDEXER_TOKEN

    return {
      server,
      port,
      token: token ?? undefined,
    }
  }

  /**
   * Returns Algod configuration derived from environment variables.
   * @throws Error if required environment variables are missing.
   */
  static getAlgodConfigFromEnvironment(): AlgoClientConfig {
    const server = process.env.ALGOD_SERVER
    if (!server) {
      throw new Error('ALGOD_SERVER environment variable not found')
    }

    const port = this.parsePort(process.env.ALGOD_PORT)
    const token = process.env.ALGOD_TOKEN

    return {
      server,
      port,
      token: token ?? undefined,
    }
  }

  /**
   * Returns KMD configuration derived from environment variables.
   * Falls back to ALGOD_* variables if KMD_* are not provided.
   * @throws Error if no server can be determined.
   */
  static getKmdConfigFromEnvironment(fallbackAlgodConfig?: AlgoClientConfig): AlgoClientConfig {
    const server = process.env.KMD_SERVER ?? fallbackAlgodConfig?.server ?? process.env.ALGOD_SERVER
    if (!server) {
      throw new Error('KMD_SERVER environment variable not found')
    }

    const port = this.parsePort(process.env.KMD_PORT)
      ?? fallbackAlgodConfig?.port
      ?? this.parsePort(process.env.ALGOD_PORT)
      ?? 4002

    const token = process.env.KMD_TOKEN ?? process.env.ALGOD_TOKEN

    return {
      server,
      port,
      token: token ?? undefined,
    }
  }

  /**
   * Returns an Algonode configuration for the provided network and service.
   */
  static getAlgonodeConfig(network: string, service: AlgorandService): AlgoClientConfig {
    if (service === AlgorandService.Kmd) {
      throw new Error('KMD is not available on algonode')
    }

    const subdomain = service === AlgorandService.Algod ? 'api' : 'idx'

    return {
      server: `https://${network}-${subdomain}.4160.nodely.dev`,
      port: 443,
    }
  }

  /**
   * Returns a default localnet configuration for the provided service.
   */
  static getDefaultLocalnetConfig(service: AlgorandService): AlgoClientConfig {
    const port = service === AlgorandService.Algod ? 4001 : service === AlgorandService.Indexer ? 8980 : 4002

    return {
      server: 'http://localhost',
      port,
      token: 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
    }
  }

  /**
   * Creates an Algod client for the given configuration.
   */
  static getAlgodClient(config: AlgoClientConfig): AlgodClient {
    const clientConfig = this.createHttpClientConfig(config, 'X-Algo-API-Token')
    return new AlgodClient(clientConfig)
  }

  /**
   * Creates an Indexer client for the given configuration.
   */
  static getIndexerClient(config: AlgoClientConfig): IndexerClient {
    const clientConfig = this.createHttpClientConfig(config, 'X-Indexer-API-Token')
    return new IndexerClient(clientConfig)
  }

  /**
   * Creates a KMD client for the given configuration.
   */
  static getKmdClient(config: AlgoClientConfig): KmdClient {
    const clientConfig = this.createHttpClientConfig(config, 'X-KMD-API-Token')
    return new KmdClient(clientConfig)
  }

  /**
   * Creates an Algod client from environment variables.
   */
  static getAlgodClientFromEnvironment(): AlgodClient {
    return this.getAlgodClient(this.getAlgodConfigFromEnvironment())
  }

  /**
   * Creates an Indexer client from environment variables.
   */
  static getIndexerClientFromEnvironment(): IndexerClient {
    return this.getIndexerClient(this.getIndexerConfigFromEnvironment())
  }

  /**
   * Creates a KMD client from environment variables.
   */
  static getKmdClientFromEnvironment(): KmdClient {
    const algodConfig = this.safeGetConfig(this.getAlgodConfigFromEnvironment.bind(this))
    return this.getKmdClient(this.getKmdConfigFromEnvironment(algodConfig))
  }

  private async fetchNetworkDetails(): Promise<NetworkDetails> {
    try {
      const params = await this.algodClient.transactionParams()
      const genesisId = params.genesisId ?? 'unknown'
      const genesisHash = Buffer.from(params.genesisHash ?? new Uint8Array()).toString('base64')

      return {
        isTestnet: genesisId === 'testnet-v1.0',
        isMainnet: genesisId === 'mainnet-v1.0',
        isLocalnet: genesisIdIsLocalNet(genesisId),
        genesisId,
        genesisHash,
      }
    } catch (error) {
      if (error instanceof ApiError) {
        throw new Error(`Failed to fetch network details: ${error.message}`)
      }
      throw error
    }
  }

  private static createHttpClientConfig(config: AlgoClientConfig, defaultHeaderName: string) {
    const baseUrl = this.buildBaseUrl(config)
    const headers = this.buildHeaders(config.token, defaultHeaderName)
    return headers ? { baseUrl, headers } : { baseUrl }
  }

  private static buildHeaders(token: TokenHeader | undefined, defaultHeaderName: string): Record<string, string> | undefined {
    if (!token) {
      return undefined
    }

    if (typeof token === 'string') {
      return { [defaultHeaderName]: token }
    }

    return { ...token }
  }

  private static buildBaseUrl(config: AlgoClientConfig): string {
    const { server, port } = config
    if (port === undefined || port === null || port === '') {
      return server
    }

    const portString = typeof port === 'string' ? port : port.toString()

    try {
      const url = new URL(server)
      url.port = portString
      const normalized = url.toString()
      return normalized.endsWith('/') ? normalized.slice(0, -1) : normalized
    } catch {
      if (/:[0-9]+$/.test(server)) {
        return server
      }
      return `${server}:${portString}`
    }
  }

  private static parsePort(value: string | number | undefined | null): number | undefined {
    if (value === undefined || value === null || value === '') {
      return undefined
    }
    if (typeof value === 'number') {
      return value
    }
    const parsed = Number(value)
    return Number.isNaN(parsed) ? undefined : parsed
  }

  private static safeGetConfig(getter: () => AlgoClientConfig): AlgoClientConfig | undefined {
    try {
      return getter()
    } catch {
      return undefined
    }
  }
}
