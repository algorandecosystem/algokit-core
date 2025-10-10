import { AppManager } from './clients/app-manager'
import { AssetManager } from './clients/asset-manager'
import { ClientManager } from './clients/client-manager'
import { AlgoConfig, AlgorandService } from './clients/network-client'
import {
  TransactionComposer,
  TransactionComposerConfig,
  TransactionComposerParams,
} from './transactions/composer'
import { SignerGetter, TransactionSigner } from './transactions/common'
import type { TransactionParams } from '@algorandfoundation/algod-client'

export type AlgorandClientParams =
  | {
      clientConfig: AlgoConfig
      clientManager?: undefined
      composerConfig?: TransactionComposerConfig
    }
  | {
      clientConfig?: undefined
      clientManager: ClientManager
      composerConfig?: TransactionComposerConfig
    }

class SignerRegistry implements SignerGetter {
  private readonly signers = new Map<string, TransactionSigner>()
  private defaultSigner?: TransactionSigner

  setSigner(address: string, signer: TransactionSigner) {
    this.signers.set(address, signer)
  }

  clearSigner(address: string) {
    this.signers.delete(address)
  }

  setDefaultSigner(signer: TransactionSigner) {
    this.defaultSigner = signer
  }

  clearDefaultSigner() {
    this.defaultSigner = undefined
  }

  getSigner(address: string): TransactionSigner {
    const signer = this.signers.get(address) ?? this.defaultSigner
    if (!signer) {
      throw new Error(`No signer registered for address ${address}. Use setSigner or setDefaultSigner to configure one.`)
    }
    return signer
  }
}

/**
 * A client that brokers easy access to Algorand functionality.
 */
export class AlgorandClient {
  private readonly clientManager: ClientManager
  private readonly assetManager: AssetManager
  private readonly appManager: AppManager
  private readonly signerRegistry = new SignerRegistry()
  private readonly defaultComposerConfig?: TransactionComposerConfig

  constructor(params: AlgorandClientParams) {
    if (!params.clientManager && !params.clientConfig) {
      throw new Error('AlgorandClient requires either a client configuration or an existing ClientManager instance.')
    }

    this.clientManager = params.clientManager ?? new ClientManager(params.clientConfig)
    this.defaultComposerConfig = params.composerConfig

    this.assetManager = new AssetManager(this.clientManager.algod, () => this.newComposer())
    this.appManager = new AppManager(this.clientManager.algod)
  }

  /** Creates a new transaction composer pre-configured with the Algorand client context. */
  newComposer(composerConfig?: TransactionComposerConfig): TransactionComposer {
    const params: TransactionComposerParams = {
      algodClient: this.clientManager.algod,
      signerGetter: this.signerRegistry,
      composerConfig: composerConfig ?? this.defaultComposerConfig,
    }
    return new TransactionComposer(params)
  }

  /** Registers a signer for a specific address. */
  setSigner(address: string, signer: TransactionSigner): this {
    this.signerRegistry.setSigner(address, signer)
    return this
  }

  /** Removes a previously registered signer for an address. */
  clearSigner(address: string): this {
    this.signerRegistry.clearSigner(address)
    return this
  }

  /** Registers a default signer used when no address-specific signer exists. */
  setDefaultSigner(signer: TransactionSigner): this {
    this.signerRegistry.setDefaultSigner(signer)
    return this
  }

  /** Clears the default signer. */
  clearDefaultSigner(): this {
    this.signerRegistry.clearDefaultSigner()
    return this
  }

  /** Returns the underlying ClientManager. */
  get client(): ClientManager {
    return this.clientManager
  }

  /** Returns the AssetManager helper. */
  get asset(): AssetManager {
    return this.assetManager
  }

  /** Returns the AppManager helper. */
  get app(): AppManager {
    return this.appManager
  }

  /** Retrieves suggested transaction parameters from algod. */
  async getSuggestedParams(): Promise<TransactionParams> {
    return await this.clientManager.algod.transactionParams()
  }

  /** Creates an AlgorandClient from a raw network configuration. */
  static fromConfig(clientConfig: AlgoConfig, composerConfig?: TransactionComposerConfig): AlgorandClient {
    return new AlgorandClient({ clientConfig, composerConfig })
  }

  /** Creates an AlgorandClient from an existing ClientManager. */
  static fromClientManager(clientManager: ClientManager, composerConfig?: TransactionComposerConfig): AlgorandClient {
    return new AlgorandClient({ clientManager, composerConfig })
  }

  /** Creates an AlgorandClient configured for a local development network. */
  static localnet(composerConfig?: TransactionComposerConfig): AlgorandClient {
    return AlgorandClient.fromConfig({
      algodConfig: ClientManager.getDefaultLocalnetConfig(AlgorandService.Algod),
      indexerConfig: ClientManager.getDefaultLocalnetConfig(AlgorandService.Indexer),
      kmdConfig: ClientManager.getDefaultLocalnetConfig(AlgorandService.Kmd),
    }, composerConfig)
  }

  /** Creates an AlgorandClient configured for Algonode TestNet. */
  static testnet(composerConfig?: TransactionComposerConfig): AlgorandClient {
    return AlgorandClient.fromConfig({
      algodConfig: ClientManager.getAlgonodeConfig('testnet', AlgorandService.Algod),
      indexerConfig: ClientManager.getAlgonodeConfig('testnet', AlgorandService.Indexer),
      kmdConfig: undefined,
    }, composerConfig)
  }

  /** Creates an AlgorandClient configured for Algonode MainNet. */
  static mainnet(composerConfig?: TransactionComposerConfig): AlgorandClient {
    return AlgorandClient.fromConfig({
      algodConfig: ClientManager.getAlgonodeConfig('mainnet', AlgorandService.Algod),
      indexerConfig: ClientManager.getAlgonodeConfig('mainnet', AlgorandService.Indexer),
      kmdConfig: undefined,
    }, composerConfig)
  }

  /** Creates an AlgorandClient from environment configuration or defaults to localnet. */
  static fromEnvironment(composerConfig?: TransactionComposerConfig): AlgorandClient {
    return AlgorandClient.fromConfig(ClientManager.getConfigFromEnvironmentOrLocalNet(), composerConfig)
  }
}
