import { AccountAssetInformation, AlgodClient, ApiError } from '@algorandfoundation/algod-client'
import { AssetOptInParams, AssetOptOutParams } from '../transactions/asset-transfer'
import { TransactionComposer } from '../transactions/composer'

/** Individual result from performing a bulk opt-in or bulk opt-out for an account against a series of assets. */
export interface BulkAssetOptInOutResult {
  /** The ID of the asset opted into / out of */
  assetId: bigint
  /** The transaction ID of the resulting opt in / out */
  transactionId: string
}

/** Information about an Algorand Standard Asset (ASA).
 *
 * This type provides a flattened, developer-friendly interface to asset information
 * that aligns with TypeScript and Python implementations.
 */
export interface AssetInformation {
  /** The ID of the asset. */
  assetId: bigint

  /** The address of the account that created the asset. */
  creator: string

  /** The total amount of the smallest divisible (decimal) units that were created of the asset. */
  total: bigint

  /** The amount of decimal places the asset was created with. */
  decimals: number

  /** Whether the asset was frozen by default for all accounts. */
  defaultFrozen?: boolean

  /** The address of the optional account that can manage the configuration of the asset and destroy it. */
  manager?: string

  /** The address of the optional account that holds the reserve (uncirculated supply) units of the asset. */
  reserve?: string

  /** The address of the optional account that can be used to freeze or unfreeze holdings of this asset for any account. */
  freeze?: string

  /** The address of the optional account that can clawback holdings of this asset from any account. */
  clawback?: string

  /** The optional name of the unit of this asset (e.g. ticker name). */
  unitName?: string

  /** The optional name of the unit of this asset as bytes. */
  unitNameB64?: Uint8Array

  /** The optional name of the asset. */
  assetName?: string

  /** The optional name of the asset as bytes. */
  assetNameB64?: Uint8Array

  /** Optional URL where more information about the asset can be retrieved (e.g. metadata). */
  url?: string

  /** Optional URL where more information about the asset can be retrieved as bytes. */
  urlB64?: Uint8Array

  /** 32-byte hash of some metadata that is relevant to the asset and/or asset holders. */
  metadataHash?: Uint8Array
}

export type AssetManagerErrorCode =
  | 'ALGOD_CLIENT_ERROR'
  | 'COMPOSER_ERROR'
  | 'ASSET_NOT_FOUND'
  | 'ACCOUNT_NOT_FOUND'
  | 'NOT_OPTED_IN'
  | 'NON_ZERO_BALANCE'

export class AssetManagerError extends Error {
  readonly code: AssetManagerErrorCode
  readonly details?: Record<string, unknown>

  constructor(code: AssetManagerErrorCode, message: string, details?: Record<string, unknown>, cause?: unknown) {
    super(message)
    this.name = 'AssetManagerError'
    this.code = code
    this.details = details
    if (cause !== undefined) {
      ;(this as { cause?: unknown }).cause = cause
    }
  }
}

/** Manages Algorand Standard Assets. */
export class AssetManager {
  private readonly algodClient: AlgodClient
  private readonly newComposer: () => TransactionComposer

  constructor(algodClient: AlgodClient, newComposer: () => TransactionComposer) {
    this.algodClient = algodClient
    this.newComposer = newComposer
  }

  /** Get asset information by asset ID. Returns a convenient, flattened view of the asset information. */
  async getById(assetId: bigint): Promise<AssetInformation> {
    try {
      const asset = await this.algodClient.getAssetById(assetId)

      return {
        assetId: asset.index,
        creator: asset.params.creator,
        total: asset.params.total,
        decimals: Number(asset.params.decimals),
        defaultFrozen: asset.params.defaultFrozen,
        manager: asset.params.manager,
        reserve: asset.params.reserve,
        freeze: asset.params.freeze,
        clawback: asset.params.clawback,
        unitName: asset.params.unitName,
        unitNameB64: asset.params.unitNameB64,
        assetName: asset.params.name,
        assetNameB64: asset.params.nameB64,
        url: asset.params.url,
        urlB64: asset.params.urlB64,
        metadataHash: asset.params.metadataHash,
      }
    } catch (error) {
      if (error instanceof ApiError && error.status === 404) {
        throw new AssetManagerError('ASSET_NOT_FOUND', `Asset not found: ${assetId}`, { assetId }, error)
      }
      throw new AssetManagerError('ALGOD_CLIENT_ERROR', 'Failed to fetch asset information', { assetId }, error)
    }
  }

  /** Get account's asset information. Returns the raw algod AccountAssetInformation type. */
  async getAccountInformation(sender: string, assetId: bigint): Promise<AccountAssetInformation> {
    try {
      return await this.algodClient.accountAssetInformation(sender, assetId)
    } catch (error) {
      if (error instanceof ApiError) {
        if (error.status === 404) {
          throw new AssetManagerError('NOT_OPTED_IN', `Account ${sender} is not opted into asset ${assetId}`, {
            sender,
            assetId,
          }, error)
        }
        if (error.status === 400) {
          throw new AssetManagerError('ACCOUNT_NOT_FOUND', `Account not found: ${sender}`, { sender }, error)
        }
      }
      throw new AssetManagerError('ALGOD_CLIENT_ERROR', 'Failed to fetch account asset information', { sender, assetId }, error)
    }
  }

  async bulkOptIn(account: string, assetIds: bigint[]): Promise<BulkAssetOptInOutResult[]> {
    if (assetIds.length === 0) {
      return []
    }

    const composer = this.newComposer()

    for (const rawAssetId of assetIds) {
      const assetId = BigInt(rawAssetId)
      const params: AssetOptInParams = {
        sender: account,
        assetId,
      }

      try {
        composer.addAssetOptIn(params)
      } catch (error) {
        throw new AssetManagerError('COMPOSER_ERROR', `Failed to add opt-in for asset ${assetId}`, { assetId }, error)
      }
    }

    try {
      const result = await composer.send()

      if (result.results.length !== assetIds.length) {
        throw new AssetManagerError(
          'COMPOSER_ERROR',
          'Composer returned an unexpected number of results',
          {
            expected: assetIds.length,
            actual: result.results.length,
          },
        )
      }

      return assetIds.map((rawAssetId, index) => ({
        assetId: BigInt(rawAssetId),
        transactionId: result.results[index].transactionId,
      }))
    } catch (error) {
      if (error instanceof AssetManagerError && error.code === 'COMPOSER_ERROR') {
        throw error
      }
      throw new AssetManagerError('COMPOSER_ERROR', 'Failed to submit opt-in transactions', undefined, error)
    }
  }

  async bulkOptOut(account: string, assetIds: bigint[], ensureZeroBalance?: boolean): Promise<BulkAssetOptInOutResult[]> {
    if (assetIds.length === 0) {
      return []
    }

    const shouldCheckBalance = ensureZeroBalance ?? false

    if (shouldCheckBalance) {
      for (const rawAssetId of assetIds) {
        const assetId = BigInt(rawAssetId)
        const accountInfo = await this.getAccountInformation(account, assetId).catch((error: unknown) => {
          if (error instanceof AssetManagerError && error.code === 'NOT_OPTED_IN') {
            throw new AssetManagerError('NOT_OPTED_IN', `Account ${account} is not opted into asset ${assetId}`, {
              account,
              assetId,
            }, error)
          }
          throw error
        })

        const balance = accountInfo.assetHolding?.amount ?? 0n
        if (balance > 0n) {
          throw new AssetManagerError('NON_ZERO_BALANCE', `Account ${account} has non-zero balance for asset ${assetId}`, {
            account,
            assetId,
            balance,
          })
        }
      }
    }

    const creators: string[] = []
    for (const rawAssetId of assetIds) {
      const assetId = BigInt(rawAssetId)
      const assetInfo = await this.getById(assetId)
      creators.push(assetInfo.creator)
    }

    const composer = this.newComposer()

    assetIds.forEach((rawAssetId, index) => {
      const assetId = BigInt(rawAssetId)
      const params: AssetOptOutParams = {
        sender: account,
        assetId,
        closeRemainderTo: creators[index],
      }

      try {
        composer.addAssetOptOut(params)
      } catch (error) {
        throw new AssetManagerError('COMPOSER_ERROR', `Failed to add opt-out for asset ${assetId}`, { assetId }, error)
      }
    })

    try {
      const result = await composer.send()

      if (result.results.length !== assetIds.length) {
        throw new AssetManagerError(
          'COMPOSER_ERROR',
          'Composer returned an unexpected number of results',
          {
            expected: assetIds.length,
            actual: result.results.length,
          },
        )
      }

      return assetIds.map((rawAssetId, index) => ({
        assetId: BigInt(rawAssetId),
        transactionId: result.results[index].transactionId,
      }))
    } catch (error) {
      if (error instanceof AssetManagerError && error.code === 'COMPOSER_ERROR') {
        throw error
      }
      throw new AssetManagerError('COMPOSER_ERROR', 'Failed to submit opt-out transactions', undefined, error)
    }
  }
}
