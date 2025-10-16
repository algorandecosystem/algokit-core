import { AccountAssetInformation, AlgodClient, ApiError } from '@algorandfoundation/algod-client'
import { AssetOptInParams, AssetOptOutParams } from '../transactions/asset-transfer'
import { TransactionComposer } from '../transactions/composer'
import { MAX_TX_GROUP_SIZE } from '@algorandfoundation/algokit-common'

const chunkArray = <T>(items: T[], size: number): T[][] => {
  if (size <= 0) {
    throw new Error('Chunk size must be greater than zero')
  }
  if (items.length <= size) {
    return [items.slice()]
  }
  const chunks: T[][] = []
  for (let index = 0; index < items.length; index += size) {
    chunks.push(items.slice(index, index + size))
  }
  return chunks
}

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

  /** The address of the account that created the asset.
   *
   * This is the address where the parameters for this asset can be found,
   * and also the address where unwanted asset units can be sent when
   * closing out an asset position and opting-out of the asset.
   */
  creator: string

  /** The total amount of the smallest divisible (decimal) units that were created of the asset.
   *
   * For example, if `decimals` is, say, 2, then for every 100 `total` there is 1 whole unit.
   */
  total: bigint

  /** The amount of decimal places the asset was created with.
   *
   * * If 0, the asset is not divisible;
   * * If 1, the base unit of the asset is in tenths;
   * * If 2, the base unit of the asset is in hundredths;
   * * If 3, the base unit of the asset is in thousandths;
   * * and so on up to 19 decimal places.
   */
  decimals: number

  /** Whether the asset was frozen by default for all accounts.
   *
   * If `true` then for anyone apart from the creator to hold the
   * asset it needs to be unfrozen per account using an asset freeze
   * transaction from the `freeze` account.
   */
  defaultFrozen?: boolean

  /** The address of the optional account that can manage the configuration of the asset and destroy it.
   *
   * If not set the asset is permanently immutable.
   */
  manager?: string

  /** The address of the optional account that holds the reserve (uncirculated supply) units of the asset.
   *
   * This address has no specific authority in the protocol itself and is informational only.
   *
   * Some standards like [ARC-19](https://github.com/algorandfoundation/ARCs/blob/main/ARCs/arc-0019.md)
   * rely on this field to hold meaningful data.
   *
   * It can be used in the case where you want to signal to holders of your asset that the uncirculated units
   * of the asset reside in an account that is different from the default creator account.
   *
   * If not set the field is permanently empty.
   */
  reserve?: string

  /** The address of the optional account that can be used to freeze or unfreeze holdings of this asset for any account.
   *
   * If empty, freezing is not permitted.
   *
   * If not set the field is permanently empty.
   */
  freeze?: string

  /** The address of the optional account that can clawback holdings of this asset from any account.
   *
   * The clawback account has the ability to **unconditionally take assets from any account**.
   *
   * If empty, clawback is not permitted.
   *
   * If not set the field is permanently empty.
   */
  clawback?: string

  /** The optional name of the unit of this asset (e.g. ticker name).
   *
   * Max size is 8 bytes.
   */
  unitName?: string

  /** The optional name of the unit of this asset as bytes.
   *
   * Max size is 8 bytes.
   */
  unitNameAsBytes?: Uint8Array

  /** The optional name of the asset.
   *
   * Max size is 32 bytes.
   */
  assetName?: string

  /** The optional name of the asset as bytes.
   *
   * Max size is 32 bytes.
   */
  assetNameAsBytes?: Uint8Array

  /** Optional URL where more information about the asset can be retrieved (e.g. metadata).
   *
   * Max size is 96 bytes.
   */
  url?: string

  /** Optional URL where more information about the asset can be retrieved as bytes.
   *
   * Max size is 96 bytes.
   */
  urlAsBytes?: Uint8Array

  /** 32-byte hash of some metadata that is relevant to the asset and/or asset holders.
   *
   * The format of this metadata is up to the application.
   */
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

  /** Get asset information by asset ID
   * Returns a convenient, flattened view of the asset information.
   */
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
        unitNameAsBytes: asset.params.unitNameB64,
        assetName: asset.params.name,
        assetNameAsBytes: asset.params.nameB64,
        url: asset.params.url,
        urlAsBytes: asset.params.urlB64,
        metadataHash: asset.params.metadataHash,
      }
    } catch (error) {
      if (error instanceof ApiError && error.status === 404) {
        throw new AssetManagerError('ASSET_NOT_FOUND', `Asset not found: ${assetId}`, { assetId }, error)
      }
      throw new AssetManagerError('ALGOD_CLIENT_ERROR', 'Failed to fetch asset information', { assetId }, error)
    }
  }

  /** Get account's asset information.
   * Returns the raw algod AccountAssetInformation type.
   * Access asset holding via `account_info.asset_holding` and asset params via `account_info.asset_params`.
   */
  async getAccountInformation(sender: string, assetId: bigint): Promise<AccountAssetInformation> {
    try {
      return await this.algodClient.accountAssetInformation(sender, assetId, { format: 'json' })
    } catch (error) {
      if (error instanceof ApiError) {
        if (error.status === 404) {
          throw new AssetManagerError(
            'NOT_OPTED_IN',
            `Account ${sender} is not opted into asset ${assetId}`,
            {
              sender,
              assetId,
            },
            error,
          )
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

    const results: BulkAssetOptInOutResult[] = []

    for (const batch of chunkArray(assetIds, MAX_TX_GROUP_SIZE)) {
      const composer = this.newComposer()

      for (const assetId of batch) {
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

        if (result.results.length !== batch.length) {
          throw new AssetManagerError('COMPOSER_ERROR', 'Composer returned an unexpected number of results', {
            expected: batch.length,
            actual: result.results.length,
          })
        }

        batch.forEach((assetId, index) => {
          results.push({
            assetId,
            transactionId: result.results[index].transactionId,
          })
        })
      } catch (error) {
        if (error instanceof AssetManagerError && error.code === 'COMPOSER_ERROR') {
          throw error
        }
        throw new AssetManagerError('COMPOSER_ERROR', 'Failed to submit opt-in transactions', undefined, error)
      }
    }

    return results
  }

  async bulkOptOut(account: string, assetIds: bigint[], ensureZeroBalance?: boolean): Promise<BulkAssetOptInOutResult[]> {
    if (assetIds.length === 0) {
      return []
    }

    const shouldCheckBalance = ensureZeroBalance ?? false
    const results: BulkAssetOptInOutResult[] = []

    if (shouldCheckBalance) {
      for (const assetId of assetIds) {
        const accountInfo = await this.getAccountInformation(account, assetId).catch((error: unknown) => {
          if (error instanceof AssetManagerError && error.code === 'NOT_OPTED_IN') {
            throw new AssetManagerError(
              'NOT_OPTED_IN',
              `Account ${account} is not opted into asset ${assetId}`,
              {
                account,
                assetId,
              },
              error,
            )
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

    const creatorCache = new Map<bigint, string>()

    for (const batch of chunkArray(assetIds, MAX_TX_GROUP_SIZE)) {
      const composer = this.newComposer()

      const creators: string[] = []
      for (const assetId of batch) {
        if (!creatorCache.has(assetId)) {
          const assetInfo = await this.getById(assetId)
          creatorCache.set(assetId, assetInfo.creator)
        }
        creators.push(creatorCache.get(assetId)!)
      }

      batch.forEach((assetId, index) => {
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

        if (result.results.length !== batch.length) {
          throw new AssetManagerError('COMPOSER_ERROR', 'Composer returned an unexpected number of results', {
            expected: batch.length,
            actual: result.results.length,
          })
        }

        batch.forEach((assetId, index) => {
          results.push({
            assetId,
            transactionId: result.results[index].transactionId,
          })
        })
      } catch (error) {
        if (error instanceof AssetManagerError && error.code === 'COMPOSER_ERROR') {
          throw error
        }
        throw new AssetManagerError('COMPOSER_ERROR', 'Failed to submit opt-out transactions', undefined, error)
      }
    }

    return results
  }
}
