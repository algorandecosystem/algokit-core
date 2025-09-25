import type { BaseHttpRequest, ApiRequestOptions } from '../core/BaseHttpRequest'
import type {
  Block,
  Box,
  HealthCheck,
  LookupAccountAppLocalStates,
  LookupAccountAssets,
  LookupAccountById,
  LookupAccountCreatedApplications,
  LookupAccountCreatedAssets,
  LookupAccountTransactions,
  LookupApplicationById,
  LookupApplicationLogsById,
  LookupAssetBalances,
  LookupAssetById,
  LookupAssetTransactions,
  LookupTransaction,
  SearchForAccounts,
  SearchForApplicationBoxes,
  SearchForApplications,
  SearchForAssets,
  SearchForBlockHeaders,
  SearchForTransactions,
} from '../models/index'
import { Block as BlockCodecs } from '../models/block'
import { Box as BoxCodecs } from '../models/box'
import { HealthCheck as HealthCheckCodecs } from '../models/health-check'
import { LookupAccountAppLocalStates as LookupAccountAppLocalStatesCodecs } from '../models/lookup-account-app-local-states'
import { LookupAccountAssets as LookupAccountAssetsCodecs } from '../models/lookup-account-assets'
import { LookupAccountById as LookupAccountByIdCodecs } from '../models/lookup-account-by-id'
import { LookupAccountCreatedApplications as LookupAccountCreatedApplicationsCodecs } from '../models/lookup-account-created-applications'
import { LookupAccountCreatedAssets as LookupAccountCreatedAssetsCodecs } from '../models/lookup-account-created-assets'
import { LookupAccountTransactions as LookupAccountTransactionsCodecs } from '../models/lookup-account-transactions'
import { LookupApplicationById as LookupApplicationByIdCodecs } from '../models/lookup-application-by-id'
import { LookupApplicationLogsById as LookupApplicationLogsByIdCodecs } from '../models/lookup-application-logs-by-id'
import { LookupAssetBalances as LookupAssetBalancesCodecs } from '../models/lookup-asset-balances'
import { LookupAssetById as LookupAssetByIdCodecs } from '../models/lookup-asset-by-id'
import { LookupAssetTransactions as LookupAssetTransactionsCodecs } from '../models/lookup-asset-transactions'
import { LookupTransaction as LookupTransactionCodecs } from '../models/lookup-transaction'
import { SearchForAccounts as SearchForAccountsCodecs } from '../models/search-for-accounts'
import { SearchForApplicationBoxes as SearchForApplicationBoxesCodecs } from '../models/search-for-application-boxes'
import { SearchForApplications as SearchForApplicationsCodecs } from '../models/search-for-applications'
import { SearchForAssets as SearchForAssetsCodecs } from '../models/search-for-assets'
import { SearchForBlockHeaders as SearchForBlockHeadersCodecs } from '../models/search-for-block-headers'
import { SearchForTransactions as SearchForTransactionsCodecs } from '../models/search-for-transactions'

export class IndexerApi {
  constructor(public readonly httpRequest: BaseHttpRequest) {}

  /**
   * Lookup an account's asset holdings, optionally for a specific ID.
   */
  async lookupAccountAppLocalStates(
    accountId: string,
    params?: { applicationId?: number | bigint; includeAll?: boolean; limit?: number | bigint; next?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAccountAppLocalStates> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/accounts/{account-id}/apps-local-state',
      path: { 'account-id': accountId },
      query: {
        'application-id': typeof params?.applicationId === 'bigint' ? (params!.applicationId as bigint).toString() : params?.applicationId,
        'include-all': params?.includeAll,
        limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit,
        next: params?.next,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as LookupAccountAppLocalStates
    try {
      return (LookupAccountAppLocalStatesCodecs as any).decodeJson(parsed as any) as LookupAccountAppLocalStates
    } catch {}
    return parsed
  }

  /**
   * Lookup an account's asset holdings, optionally for a specific ID.
   */
  async lookupAccountAssets(
    accountId: string,
    params?: { assetId?: number | bigint; includeAll?: boolean; limit?: number | bigint; next?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAccountAssets> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/accounts/{account-id}/assets',
      path: { 'account-id': accountId },
      query: {
        'asset-id': typeof params?.assetId === 'bigint' ? (params!.assetId as bigint).toString() : params?.assetId,
        'include-all': params?.includeAll,
        limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit,
        next: params?.next,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as LookupAccountAssets
    try {
      return (LookupAccountAssetsCodecs as any).decodeJson(parsed as any) as LookupAccountAssets
    } catch {}
    return parsed
  }

  /**
   * Lookup account information.
   */
  async lookupAccountById(
    accountId: string,
    params?: {
      round?: number | bigint
      includeAll?: boolean
      exclude?: 'all' | 'assets' | 'created-assets' | 'apps-local-state' | 'created-apps' | 'none'[]
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAccountById> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/accounts/{account-id}',
      path: { 'account-id': accountId },
      query: {
        round: typeof params?.round === 'bigint' ? (params!.round as bigint).toString() : params?.round,
        'include-all': params?.includeAll,
        exclude: params?.exclude,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as LookupAccountById
    try {
      return (LookupAccountByIdCodecs as any).decodeJson(parsed as any) as LookupAccountById
    } catch {}
    return parsed
  }

  /**
   * Lookup an account's created application parameters, optionally for a specific ID.
   */
  async lookupAccountCreatedApplications(
    accountId: string,
    params?: { applicationId?: number | bigint; includeAll?: boolean; limit?: number | bigint; next?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAccountCreatedApplications> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/accounts/{account-id}/created-applications',
      path: { 'account-id': accountId },
      query: {
        'application-id': typeof params?.applicationId === 'bigint' ? (params!.applicationId as bigint).toString() : params?.applicationId,
        'include-all': params?.includeAll,
        limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit,
        next: params?.next,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as LookupAccountCreatedApplications
    try {
      return (LookupAccountCreatedApplicationsCodecs as any).decodeJson(parsed as any) as LookupAccountCreatedApplications
    } catch {}
    return parsed
  }

  /**
   * Lookup an account's created asset parameters, optionally for a specific ID.
   */
  async lookupAccountCreatedAssets(
    accountId: string,
    params?: { assetId?: number | bigint; includeAll?: boolean; limit?: number | bigint; next?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAccountCreatedAssets> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/accounts/{account-id}/created-assets',
      path: { 'account-id': accountId },
      query: {
        'asset-id': typeof params?.assetId === 'bigint' ? (params!.assetId as bigint).toString() : params?.assetId,
        'include-all': params?.includeAll,
        limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit,
        next: params?.next,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as LookupAccountCreatedAssets
    try {
      return (LookupAccountCreatedAssetsCodecs as any).decodeJson(parsed as any) as LookupAccountCreatedAssets
    } catch {}
    return parsed
  }

  /**
   * Lookup account transactions. Transactions are returned newest to oldest.
   */
  async lookupAccountTransactions(
    accountId: string,
    params?: {
      limit?: number | bigint
      next?: string
      notePrefix?: string
      txType?: 'pay' | 'keyreg' | 'acfg' | 'axfer' | 'afrz' | 'appl' | 'stpf' | 'hb'
      sigType?: 'sig' | 'msig' | 'lsig'
      txid?: string
      round?: number | bigint
      minRound?: number | bigint
      maxRound?: number | bigint
      assetId?: number | bigint
      beforeTime?: string
      afterTime?: string
      currencyGreaterThan?: number | bigint
      currencyLessThan?: number | bigint
      rekeyTo?: boolean
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAccountTransactions> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/accounts/{account-id}/transactions',
      path: { 'account-id': accountId },
      query: {
        limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit,
        next: params?.next,
        'note-prefix': params?.notePrefix,
        'tx-type': params?.txType,
        'sig-type': params?.sigType,
        txid: params?.txid,
        round: typeof params?.round === 'bigint' ? (params!.round as bigint).toString() : params?.round,
        'min-round': typeof params?.minRound === 'bigint' ? (params!.minRound as bigint).toString() : params?.minRound,
        'max-round': typeof params?.maxRound === 'bigint' ? (params!.maxRound as bigint).toString() : params?.maxRound,
        'asset-id': typeof params?.assetId === 'bigint' ? (params!.assetId as bigint).toString() : params?.assetId,
        'before-time': params?.beforeTime,
        'after-time': params?.afterTime,
        'currency-greater-than':
          typeof params?.currencyGreaterThan === 'bigint'
            ? (params!.currencyGreaterThan as bigint).toString()
            : params?.currencyGreaterThan,
        'currency-less-than':
          typeof params?.currencyLessThan === 'bigint' ? (params!.currencyLessThan as bigint).toString() : params?.currencyLessThan,
        'rekey-to': params?.rekeyTo,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as LookupAccountTransactions
    try {
      return (LookupAccountTransactionsCodecs as any).decodeJson(parsed as any) as LookupAccountTransactions
    } catch {}
    return parsed
  }

  /**
   * Given an application ID and box name, returns base64 encoded box name and value. Box names must be in the goal app call arg form 'encoding:value'. For ints, use the form 'int:1234'. For raw bytes, encode base 64 and use 'b64' prefix as in 'b64:A=='. For printable strings, use the form 'str:hello'. For addresses, use the form 'addr:XYZ...'.
   */
  async lookupApplicationBoxByIdandName(
    applicationId: number | bigint,
    params?: { name: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<Box> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/applications/{application-id}/box',
      path: { 'application-id': typeof applicationId === 'bigint' ? applicationId.toString() : applicationId },
      query: { name: params?.name },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as Box
    try {
      return (BoxCodecs as any).decodeJson(parsed as any) as Box
    } catch {}
    return parsed
  }

  /**
   * Lookup application.
   */
  async lookupApplicationById(
    applicationId: number | bigint,
    params?: { includeAll?: boolean },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupApplicationById> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/applications/{application-id}',
      path: { 'application-id': typeof applicationId === 'bigint' ? applicationId.toString() : applicationId },
      query: { 'include-all': params?.includeAll },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as LookupApplicationById
    try {
      return (LookupApplicationByIdCodecs as any).decodeJson(parsed as any) as LookupApplicationById
    } catch {}
    return parsed
  }

  /**
   * Lookup application logs.
   */
  async lookupApplicationLogsById(
    applicationId: number | bigint,
    params?: {
      limit?: number | bigint
      next?: string
      txid?: string
      minRound?: number | bigint
      maxRound?: number | bigint
      senderAddress?: string
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupApplicationLogsById> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/applications/{application-id}/logs',
      path: { 'application-id': typeof applicationId === 'bigint' ? applicationId.toString() : applicationId },
      query: {
        limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit,
        next: params?.next,
        txid: params?.txid,
        'min-round': typeof params?.minRound === 'bigint' ? (params!.minRound as bigint).toString() : params?.minRound,
        'max-round': typeof params?.maxRound === 'bigint' ? (params!.maxRound as bigint).toString() : params?.maxRound,
        'sender-address': params?.senderAddress,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as LookupApplicationLogsById
    try {
      return (LookupApplicationLogsByIdCodecs as any).decodeJson(parsed as any) as LookupApplicationLogsById
    } catch {}
    return parsed
  }

  /**
   * Lookup the list of accounts who hold this asset
   */
  async lookupAssetBalances(
    assetId: number | bigint,
    params?: {
      includeAll?: boolean
      limit?: number | bigint
      next?: string
      currencyGreaterThan?: number | bigint
      currencyLessThan?: number | bigint
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAssetBalances> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/assets/{asset-id}/balances',
      path: { 'asset-id': typeof assetId === 'bigint' ? assetId.toString() : assetId },
      query: {
        'include-all': params?.includeAll,
        limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit,
        next: params?.next,
        'currency-greater-than':
          typeof params?.currencyGreaterThan === 'bigint'
            ? (params!.currencyGreaterThan as bigint).toString()
            : params?.currencyGreaterThan,
        'currency-less-than':
          typeof params?.currencyLessThan === 'bigint' ? (params!.currencyLessThan as bigint).toString() : params?.currencyLessThan,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as LookupAssetBalances
    try {
      return (LookupAssetBalancesCodecs as any).decodeJson(parsed as any) as LookupAssetBalances
    } catch {}
    return parsed
  }

  /**
   * Lookup asset information.
   */
  async lookupAssetById(
    assetId: number | bigint,
    params?: { includeAll?: boolean },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAssetById> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/assets/{asset-id}',
      path: { 'asset-id': typeof assetId === 'bigint' ? assetId.toString() : assetId },
      query: { 'include-all': params?.includeAll },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as LookupAssetById
    try {
      return (LookupAssetByIdCodecs as any).decodeJson(parsed as any) as LookupAssetById
    } catch {}
    return parsed
  }

  /**
   * Lookup transactions for an asset. Transactions are returned oldest to newest.
   */
  async lookupAssetTransactions(
    assetId: number | bigint,
    params?: {
      limit?: number | bigint
      next?: string
      notePrefix?: string
      txType?: 'pay' | 'keyreg' | 'acfg' | 'axfer' | 'afrz' | 'appl' | 'stpf' | 'hb'
      sigType?: 'sig' | 'msig' | 'lsig'
      txid?: string
      round?: number | bigint
      minRound?: number | bigint
      maxRound?: number | bigint
      beforeTime?: string
      afterTime?: string
      currencyGreaterThan?: number | bigint
      currencyLessThan?: number | bigint
      address?: string
      addressRole?: 'sender' | 'receiver' | 'freeze-target'
      excludeCloseTo?: boolean
      rekeyTo?: boolean
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAssetTransactions> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/assets/{asset-id}/transactions',
      path: { 'asset-id': typeof assetId === 'bigint' ? assetId.toString() : assetId },
      query: {
        limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit,
        next: params?.next,
        'note-prefix': params?.notePrefix,
        'tx-type': params?.txType,
        'sig-type': params?.sigType,
        txid: params?.txid,
        round: typeof params?.round === 'bigint' ? (params!.round as bigint).toString() : params?.round,
        'min-round': typeof params?.minRound === 'bigint' ? (params!.minRound as bigint).toString() : params?.minRound,
        'max-round': typeof params?.maxRound === 'bigint' ? (params!.maxRound as bigint).toString() : params?.maxRound,
        'before-time': params?.beforeTime,
        'after-time': params?.afterTime,
        'currency-greater-than':
          typeof params?.currencyGreaterThan === 'bigint'
            ? (params!.currencyGreaterThan as bigint).toString()
            : params?.currencyGreaterThan,
        'currency-less-than':
          typeof params?.currencyLessThan === 'bigint' ? (params!.currencyLessThan as bigint).toString() : params?.currencyLessThan,
        address: params?.address,
        'address-role': params?.addressRole,
        'exclude-close-to': params?.excludeCloseTo,
        'rekey-to': params?.rekeyTo,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as LookupAssetTransactions
    try {
      return (LookupAssetTransactionsCodecs as any).decodeJson(parsed as any) as LookupAssetTransactions
    } catch {}
    return parsed
  }

  /**
   * Lookup block.
   */
  async lookupBlock(roundNumber: number | bigint, params?: { headerOnly?: boolean }, requestOptions?: ApiRequestOptions): Promise<Block> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/blocks/{round-number}',
      path: { 'round-number': typeof roundNumber === 'bigint' ? roundNumber.toString() : roundNumber },
      query: { 'header-only': params?.headerOnly },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as Block
    try {
      return (BlockCodecs as any).decodeJson(parsed as any) as Block
    } catch {}
    return parsed
  }

  /**
   * Lookup a single transaction.
   */
  async lookupTransaction(txid: string, requestOptions?: ApiRequestOptions): Promise<LookupTransaction> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/transactions/{txid}',
      path: { txid: txid },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as LookupTransaction
    try {
      return (LookupTransactionCodecs as any).decodeJson(parsed as any) as LookupTransaction
    } catch {}
    return parsed
  }

  async makeHealthCheck(requestOptions?: ApiRequestOptions): Promise<HealthCheck> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/health',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as HealthCheck
    try {
      return (HealthCheckCodecs as any).decodeJson(parsed as any) as HealthCheck
    } catch {}
    return parsed
  }

  /**
   * Search for accounts.
   */
  async searchForAccounts(
    params?: {
      assetId?: number | bigint
      limit?: number | bigint
      next?: string
      currencyGreaterThan?: number | bigint
      includeAll?: boolean
      exclude?: 'all' | 'assets' | 'created-assets' | 'apps-local-state' | 'created-apps' | 'none'[]
      currencyLessThan?: number | bigint
      authAddr?: string
      round?: number | bigint
      applicationId?: number | bigint
      onlineOnly?: boolean
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<SearchForAccounts> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/accounts',
      path: {},
      query: {
        'asset-id': typeof params?.assetId === 'bigint' ? (params!.assetId as bigint).toString() : params?.assetId,
        limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit,
        next: params?.next,
        'currency-greater-than':
          typeof params?.currencyGreaterThan === 'bigint'
            ? (params!.currencyGreaterThan as bigint).toString()
            : params?.currencyGreaterThan,
        'include-all': params?.includeAll,
        exclude: params?.exclude,
        'currency-less-than':
          typeof params?.currencyLessThan === 'bigint' ? (params!.currencyLessThan as bigint).toString() : params?.currencyLessThan,
        'auth-addr': params?.authAddr,
        round: typeof params?.round === 'bigint' ? (params!.round as bigint).toString() : params?.round,
        'application-id': typeof params?.applicationId === 'bigint' ? (params!.applicationId as bigint).toString() : params?.applicationId,
        'online-only': params?.onlineOnly,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as SearchForAccounts
    try {
      return (SearchForAccountsCodecs as any).decodeJson(parsed as any) as SearchForAccounts
    } catch {}
    return parsed
  }

  /**
   * Given an application ID, returns the box names of that application sorted lexicographically.
   */
  async searchForApplicationBoxes(
    applicationId: number | bigint,
    params?: { limit?: number | bigint; next?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<SearchForApplicationBoxes> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/applications/{application-id}/boxes',
      path: { 'application-id': typeof applicationId === 'bigint' ? applicationId.toString() : applicationId },
      query: { limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit, next: params?.next },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as SearchForApplicationBoxes
    try {
      return (SearchForApplicationBoxesCodecs as any).decodeJson(parsed as any) as SearchForApplicationBoxes
    } catch {}
    return parsed
  }

  /**
   * Search for applications
   */
  async searchForApplications(
    params?: { applicationId?: number | bigint; creator?: string; includeAll?: boolean; limit?: number | bigint; next?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<SearchForApplications> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/applications',
      path: {},
      query: {
        'application-id': typeof params?.applicationId === 'bigint' ? (params!.applicationId as bigint).toString() : params?.applicationId,
        creator: params?.creator,
        'include-all': params?.includeAll,
        limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit,
        next: params?.next,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as SearchForApplications
    try {
      return (SearchForApplicationsCodecs as any).decodeJson(parsed as any) as SearchForApplications
    } catch {}
    return parsed
  }

  /**
   * Search for assets.
   */
  async searchForAssets(
    params?: {
      includeAll?: boolean
      limit?: number | bigint
      next?: string
      creator?: string
      name?: string
      unit?: string
      assetId?: number | bigint
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<SearchForAssets> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/assets',
      path: {},
      query: {
        'include-all': params?.includeAll,
        limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit,
        next: params?.next,
        creator: params?.creator,
        name: params?.name,
        unit: params?.unit,
        'asset-id': typeof params?.assetId === 'bigint' ? (params!.assetId as bigint).toString() : params?.assetId,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as SearchForAssets
    try {
      return (SearchForAssetsCodecs as any).decodeJson(parsed as any) as SearchForAssets
    } catch {}
    return parsed
  }

  /**
   * Search for block headers. Block headers are returned in ascending round order. Transactions are not included in the output.
   */
  async searchForBlockHeaders(
    params?: {
      limit?: number | bigint
      next?: string
      minRound?: number | bigint
      maxRound?: number | bigint
      beforeTime?: string
      afterTime?: string
      proposers?: string[]
      expired?: string[]
      absent?: string[]
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<SearchForBlockHeaders> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/block-headers',
      path: {},
      query: {
        limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit,
        next: params?.next,
        'min-round': typeof params?.minRound === 'bigint' ? (params!.minRound as bigint).toString() : params?.minRound,
        'max-round': typeof params?.maxRound === 'bigint' ? (params!.maxRound as bigint).toString() : params?.maxRound,
        'before-time': params?.beforeTime,
        'after-time': params?.afterTime,
        proposers: params?.proposers,
        expired: params?.expired,
        absent: params?.absent,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as SearchForBlockHeaders
    try {
      return (SearchForBlockHeadersCodecs as any).decodeJson(parsed as any) as SearchForBlockHeaders
    } catch {}
    return parsed
  }

  /**
   * Search for transactions. Transactions are returned oldest to newest unless the address parameter is used, in which case results are returned newest to oldest.
   */
  async searchForTransactions(
    params?: {
      limit?: number | bigint
      next?: string
      notePrefix?: string
      txType?: 'pay' | 'keyreg' | 'acfg' | 'axfer' | 'afrz' | 'appl' | 'stpf' | 'hb'
      sigType?: 'sig' | 'msig' | 'lsig'
      groupId?: string
      txid?: string
      round?: number | bigint
      minRound?: number | bigint
      maxRound?: number | bigint
      assetId?: number | bigint
      beforeTime?: string
      afterTime?: string
      currencyGreaterThan?: number | bigint
      currencyLessThan?: number | bigint
      address?: string
      addressRole?: 'sender' | 'receiver' | 'freeze-target'
      excludeCloseTo?: boolean
      rekeyTo?: boolean
      applicationId?: number | bigint
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<SearchForTransactions> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/transactions',
      path: {},
      query: {
        limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit,
        next: params?.next,
        'note-prefix': params?.notePrefix,
        'tx-type': params?.txType,
        'sig-type': params?.sigType,
        'group-id': params?.groupId,
        txid: params?.txid,
        round: typeof params?.round === 'bigint' ? (params!.round as bigint).toString() : params?.round,
        'min-round': typeof params?.minRound === 'bigint' ? (params!.minRound as bigint).toString() : params?.minRound,
        'max-round': typeof params?.maxRound === 'bigint' ? (params!.maxRound as bigint).toString() : params?.maxRound,
        'asset-id': typeof params?.assetId === 'bigint' ? (params!.assetId as bigint).toString() : params?.assetId,
        'before-time': params?.beforeTime,
        'after-time': params?.afterTime,
        'currency-greater-than':
          typeof params?.currencyGreaterThan === 'bigint'
            ? (params!.currencyGreaterThan as bigint).toString()
            : params?.currencyGreaterThan,
        'currency-less-than':
          typeof params?.currencyLessThan === 'bigint' ? (params!.currencyLessThan as bigint).toString() : params?.currencyLessThan,
        address: params?.address,
        'address-role': params?.addressRole,
        'exclude-close-to': params?.excludeCloseTo,
        'rekey-to': params?.rekeyTo,
        'application-id': typeof params?.applicationId === 'bigint' ? (params!.applicationId as bigint).toString() : params?.applicationId,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as SearchForTransactions
    try {
      return (SearchForTransactionsCodecs as any).decodeJson(parsed as any) as SearchForTransactions
    } catch {}
    return parsed
  }
}
