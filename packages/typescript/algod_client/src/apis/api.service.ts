import type { BaseHttpRequest, ApiRequestOptions } from '../core/BaseHttpRequest'
import type {
  AbortCatchup,
  Account,
  AccountApplicationInformation,
  AccountAssetInformation,
  AccountAssetsInformation,
  AddParticipationKey,
  Application,
  Asset,
  Box,
  DebugSettingsProf,
  DryrunRequest,
  Genesis,
  GetApplicationBoxes,
  GetBlockHash,
  GetBlockLogs,
  GetBlockTimeStampOffset,
  GetBlockTxids,
  GetStatus,
  GetSupply,
  GetSyncRound,
  LightBlockHeaderProof,
  ParticipationKey,
  RawTransaction,
  ShutdownNode,
  SimulateRequest,
  SimulateTransaction,
  StartCatchup,
  StateProof,
  TealCompile,
  TealDisassemble,
  TealDryrun,
  TransactionParams,
  TransactionProof,
  Version,
  WaitForBlock,
} from '../models/index'
import { AbortCatchup as AbortCatchupCodecs } from '../models/abort-catchup'
import { Account as AccountCodecs } from '../models/account'
import { AccountApplicationInformation as AccountApplicationInformationCodecs } from '../models/account-application-information'
import { AccountAssetInformation as AccountAssetInformationCodecs } from '../models/account-asset-information'
import { AccountAssetsInformation as AccountAssetsInformationCodecs } from '../models/account-assets-information'
import { AddParticipationKey as AddParticipationKeyCodecs } from '../models/add-participation-key'
import { Application as ApplicationCodecs } from '../models/application'
import { Asset as AssetCodecs } from '../models/asset'
import { Box as BoxCodecs } from '../models/box'
import { DebugSettingsProf as DebugSettingsProfCodecs } from '../models/debug-settings-prof'
import { DryrunRequest as DryrunRequestCodecs } from '../models/dryrun-request'
import { Genesis as GenesisCodecs } from '../models/genesis'
import { GetApplicationBoxes as GetApplicationBoxesCodecs } from '../models/get-application-boxes'
import { GetBlockHash as GetBlockHashCodecs } from '../models/get-block-hash'
import { GetBlockLogs as GetBlockLogsCodecs } from '../models/get-block-logs'
import { GetBlockTimeStampOffset as GetBlockTimeStampOffsetCodecs } from '../models/get-block-time-stamp-offset'
import { GetBlockTxids as GetBlockTxidsCodecs } from '../models/get-block-txids'
import { GetStatus as GetStatusCodecs } from '../models/get-status'
import { GetSupply as GetSupplyCodecs } from '../models/get-supply'
import { GetSyncRound as GetSyncRoundCodecs } from '../models/get-sync-round'
import { LightBlockHeaderProof as LightBlockHeaderProofCodecs } from '../models/light-block-header-proof'
import { ParticipationKey as ParticipationKeyCodecs } from '../models/participation-key'
import { RawTransaction as RawTransactionCodecs } from '../models/raw-transaction'
import { ShutdownNode as ShutdownNodeCodecs } from '../models/shutdown-node'
import { SimulateRequest as SimulateRequestCodecs } from '../models/simulate-request'
import { SimulateTransaction as SimulateTransactionCodecs } from '../models/simulate-transaction'
import { StartCatchup as StartCatchupCodecs } from '../models/start-catchup'
import { StateProof as StateProofCodecs } from '../models/state-proof'
import { TealCompile as TealCompileCodecs } from '../models/teal-compile'
import { TealDisassemble as TealDisassembleCodecs } from '../models/teal-disassemble'
import { TealDryrun as TealDryrunCodecs } from '../models/teal-dryrun'
import { TransactionParams as TransactionParamsCodecs } from '../models/transaction-params'
import { TransactionProof as TransactionProofCodecs } from '../models/transaction-proof'
import { Version as VersionCodecs } from '../models/version'
import { WaitForBlock as WaitForBlockCodecs } from '../models/wait-for-block'

export class AlgodApi {
  constructor(public readonly httpRequest: BaseHttpRequest) {}

  /**
   * Given a catchpoint, it aborts catching up to this catchpoint
   */
  async abortCatchup(catchpoint: string, requestOptions?: ApiRequestOptions): Promise<AbortCatchup> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'DELETE',
      url: '/v2/catchup/{catchpoint}',
      path: { catchpoint: catchpoint },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as AbortCatchup
    try {
      return (AbortCatchupCodecs as any).decodeJson(parsed as any) as AbortCatchup
    } catch {}
    return parsed
  }

  /**
   * Given a specific account public key and application ID, this call returns the account's application local state and global state (AppLocalState and AppParams, if either exists). Global state will only be returned if the provided address is the application's creator.
   */
  async accountApplicationInformation(
    address: string,
    applicationId: number | bigint,
    params?: { format?: 'json' | 'msgpack' },
    requestOptions?: ApiRequestOptions,
  ): Promise<AccountApplicationInformation> {
    const headers: Record<string, string> = {}
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === 'json'
    headers['Accept'] = useJson ? 'application/json' : 'application/msgpack'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/accounts/{address}/applications/{application-id}',
      path: { address: address, 'application-id': typeof applicationId === 'bigint' ? applicationId.toString() : applicationId },
      query: { format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    if (params?.format === 'json') {
      const parsed = (await rsp) as AccountApplicationInformation
      try {
        return (AccountApplicationInformationCodecs as any).decodeJson(parsed as any) as AccountApplicationInformation
      } catch {}
      return parsed
    } else {
      const buf = (await rsp) as unknown as Uint8Array
      try {
        return (AccountApplicationInformationCodecs as any).decodeMsgpack(buf) as AccountApplicationInformation
      } catch {}
      return buf as unknown as AccountApplicationInformation
    }
  }

  /**
   * Given a specific account public key and asset ID, this call returns the account's asset holding and asset parameters (if either exist). Asset parameters will only be returned if the provided address is the asset's creator.
   */
  async accountAssetInformation(
    address: string,
    assetId: number | bigint,
    params?: { format?: 'json' | 'msgpack' },
    requestOptions?: ApiRequestOptions,
  ): Promise<AccountAssetInformation> {
    const headers: Record<string, string> = {}
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === 'json'
    headers['Accept'] = useJson ? 'application/json' : 'application/msgpack'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/accounts/{address}/assets/{asset-id}',
      path: { address: address, 'asset-id': typeof assetId === 'bigint' ? assetId.toString() : assetId },
      query: { format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    if (params?.format === 'json') {
      const parsed = (await rsp) as AccountAssetInformation
      try {
        return (AccountAssetInformationCodecs as any).decodeJson(parsed as any) as AccountAssetInformation
      } catch {}
      return parsed
    } else {
      const buf = (await rsp) as unknown as Uint8Array
      try {
        return (AccountAssetInformationCodecs as any).decodeMsgpack(buf) as AccountAssetInformation
      } catch {}
      return buf as unknown as AccountAssetInformation
    }
  }

  /**
   * Lookup an account's asset holdings.
   */
  async accountAssetsInformation(
    address: string,
    params?: { limit?: number | bigint; next?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<AccountAssetsInformation> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/accounts/{address}/assets',
      path: { address: address },
      query: { limit: typeof params?.limit === 'bigint' ? (params!.limit as bigint).toString() : params?.limit, next: params?.next },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as AccountAssetsInformation
    try {
      return (AccountAssetsInformationCodecs as any).decodeJson(parsed as any) as AccountAssetsInformation
    } catch {}
    return parsed
  }

  /**
   * Given a specific account public key, this call returns the account's status, balance and spendable amounts
   */
  async accountInformation(
    address: string,
    params?: { exclude?: 'all' | 'none'; format?: 'json' | 'msgpack' },
    requestOptions?: ApiRequestOptions,
  ): Promise<Account> {
    const headers: Record<string, string> = {}
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === 'json'
    headers['Accept'] = useJson ? 'application/json' : 'application/msgpack'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/accounts/{address}',
      path: { address: address },
      query: { exclude: params?.exclude, format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    if (params?.format === 'json') {
      const parsed = (await rsp) as Account
      try {
        return (AccountCodecs as any).decodeJson(parsed as any) as Account
      } catch {}
      return parsed
    } else {
      const buf = (await rsp) as unknown as Uint8Array
      try {
        return (AccountCodecs as any).decodeMsgpack(buf) as Account
      } catch {}
      return buf as unknown as Account
    }
  }

  async addParticipationKey(params?: { body: string }, requestOptions?: ApiRequestOptions): Promise<AddParticipationKey> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'
    headers['Content-Type'] = 'application/msgpack'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v2/participation',
      path: {},
      query: {},
      headers,
      body: ((): any => {
        const b = params?.body as any
        if (b == null) return undefined
        const t = 'string'
        if (t in ModelCodecs) return (ModelCodecs as any)[t].encodeMsgpack(b)
        return b instanceof Uint8Array ? b : b
      })(),
      // Only msgpack supported for request body
      mediaType: 'application/msgpack',
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as AddParticipationKey
    try {
      return (AddParticipationKeyCodecs as any).decodeJson(parsed as any) as AddParticipationKey
    } catch {}
    return parsed
  }

  /**
   * Given a participation ID, append state proof keys to a particular set of participation keys
   */
  async appendKeys(participationId: string, params?: { body: string }, requestOptions?: ApiRequestOptions): Promise<ParticipationKey> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'
    headers['Content-Type'] = 'application/msgpack'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v2/participation/{participation-id}',
      path: { 'participation-id': participationId },
      query: {},
      headers,
      body: ((): any => {
        const b = params?.body as any
        if (b == null) return undefined
        const t = 'string'
        if (t in ModelCodecs) return (ModelCodecs as any)[t].encodeMsgpack(b)
        return b instanceof Uint8Array ? b : b
      })(),
      // Only msgpack supported for request body
      mediaType: 'application/msgpack',
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as ParticipationKey
    try {
      return (ParticipationKeyCodecs as any).decodeJson(parsed as any) as ParticipationKey
    } catch {}
    return parsed
  }

  /**
   * Delete a given participation key by ID
   */
  async deleteParticipationKeyById(participationId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'DELETE',
      url: '/v2/participation/{participation-id}',
      path: { 'participation-id': participationId },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as void
    try {
      return (voidCodecs as any).decodeJson(parsed as any) as void
    } catch {}
    return parsed
  }

  async experimentalCheck(requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/experimental',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as void
    try {
      return (voidCodecs as any).decodeJson(parsed as any) as void
    } catch {}
    return parsed
  }

  async generateParticipationKeys(
    address: string,
    params?: { dilution?: number | bigint; first: number | bigint; last: number | bigint },
    requestOptions?: ApiRequestOptions,
  ): Promise<string> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v2/participation/generate/{address}',
      path: { address: address },
      query: {
        dilution: typeof params?.dilution === 'bigint' ? (params!.dilution as bigint).toString() : params?.dilution,
        first: typeof params?.first === 'bigint' ? (params!.first as bigint).toString() : params?.first,
        last: typeof params?.last === 'bigint' ? (params!.last as bigint).toString() : params?.last,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as string
    try {
      return (stringCodecs as any).decodeJson(parsed as any) as string
    } catch {}
    return parsed
  }

  /**
   * Given an application ID and box name, it returns the round, box name, and value (each base64 encoded). Box names must be in the goal app call arg encoding form 'encoding:value'. For ints, use the form 'int:1234'. For raw bytes, use the form 'b64:A=='. For printable strings, use the form 'str:hello'. For addresses, use the form 'addr:XYZ...'.
   */
  async getApplicationBoxByName(
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
   * Given an application ID, return all Box names. No particular ordering is guaranteed. Request fails when client or server-side configured limits prevent returning all Box names.
   */
  async getApplicationBoxes(
    applicationId: number | bigint,
    params?: { max?: number | bigint },
    requestOptions?: ApiRequestOptions,
  ): Promise<GetApplicationBoxes> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/applications/{application-id}/boxes',
      path: { 'application-id': typeof applicationId === 'bigint' ? applicationId.toString() : applicationId },
      query: { max: typeof params?.max === 'bigint' ? (params!.max as bigint).toString() : params?.max },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as GetApplicationBoxes
    try {
      return (GetApplicationBoxesCodecs as any).decodeJson(parsed as any) as GetApplicationBoxes
    } catch {}
    return parsed
  }

  /**
   * Given a application ID, it returns application information including creator, approval and clear programs, global and local schemas, and global state.
   */
  async getApplicationById(applicationId: number | bigint, requestOptions?: ApiRequestOptions): Promise<Application> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/applications/{application-id}',
      path: { 'application-id': typeof applicationId === 'bigint' ? applicationId.toString() : applicationId },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as Application
    try {
      return (ApplicationCodecs as any).decodeJson(parsed as any) as Application
    } catch {}
    return parsed
  }

  /**
   * Given a asset ID, it returns asset information including creator, name, total supply and special addresses.
   */
  async getAssetById(assetId: number | bigint, requestOptions?: ApiRequestOptions): Promise<Asset> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/assets/{asset-id}',
      path: { 'asset-id': typeof assetId === 'bigint' ? assetId.toString() : assetId },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as Asset
    try {
      return (AssetCodecs as any).decodeJson(parsed as any) as Asset
    } catch {}
    return parsed
  }

  async getBlock(
    round: number | bigint,
    params?: { headerOnly?: boolean; format?: 'json' | 'msgpack' },
    requestOptions?: ApiRequestOptions,
  ): Promise<Uint8Array> {
    const headers: Record<string, string> = {}
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === 'json'
    headers['Accept'] = useJson ? 'application/json' : 'application/msgpack'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/blocks/{round}',
      path: { round: typeof round === 'bigint' ? round.toString() : round },
      query: { 'header-only': params?.headerOnly, format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    if (params?.format === 'json') {
      const parsed = (await rsp) as Uint8Array
      try {
        return (Uint8ArrayCodecs as any).decodeJson(parsed as any) as Uint8Array
      } catch {}
      return parsed
    } else {
      const buf = (await rsp) as unknown as Uint8Array
      try {
        return (Uint8ArrayCodecs as any).decodeMsgpack(buf) as Uint8Array
      } catch {}
      return buf as unknown as Uint8Array
    }
  }

  async getBlockHash(round: number | bigint, requestOptions?: ApiRequestOptions): Promise<GetBlockHash> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/blocks/{round}/hash',
      path: { round: typeof round === 'bigint' ? round.toString() : round },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as GetBlockHash
    try {
      return (GetBlockHashCodecs as any).decodeJson(parsed as any) as GetBlockHash
    } catch {}
    return parsed
  }

  /**
   * Get all of the logs from outer and inner app calls in the given round
   */
  async getBlockLogs(round: number | bigint, requestOptions?: ApiRequestOptions): Promise<GetBlockLogs> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/blocks/{round}/logs',
      path: { round: typeof round === 'bigint' ? round.toString() : round },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as GetBlockLogs
    try {
      return (GetBlockLogsCodecs as any).decodeJson(parsed as any) as GetBlockLogs
    } catch {}
    return parsed
  }

  /**
   * Gets the current timestamp offset.
   */
  async getBlockTimeStampOffset(requestOptions?: ApiRequestOptions): Promise<GetBlockTimeStampOffset> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/devmode/blocks/offset',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as GetBlockTimeStampOffset
    try {
      return (GetBlockTimeStampOffsetCodecs as any).decodeJson(parsed as any) as GetBlockTimeStampOffset
    } catch {}
    return parsed
  }

  async getBlockTxids(round: number | bigint, requestOptions?: ApiRequestOptions): Promise<GetBlockTxids> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/blocks/{round}/txids',
      path: { round: typeof round === 'bigint' ? round.toString() : round },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as GetBlockTxids
    try {
      return (GetBlockTxidsCodecs as any).decodeJson(parsed as any) as GetBlockTxids
    } catch {}
    return parsed
  }

  /**
   * Returns the merged (defaults + overrides) config file in json.
   */
  async getConfig(requestOptions?: ApiRequestOptions): Promise<string> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/debug/settings/config',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as string
    try {
      return (stringCodecs as any).decodeJson(parsed as any) as string
    } catch {}
    return parsed
  }

  /**
   * Retrieves the current settings for blocking and mutex profiles
   */
  async getDebugSettingsProf(requestOptions?: ApiRequestOptions): Promise<DebugSettingsProf> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/debug/settings/pprof',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as DebugSettingsProf
    try {
      return (DebugSettingsProfCodecs as any).decodeJson(parsed as any) as DebugSettingsProf
    } catch {}
    return parsed
  }

  /**
   * Returns the entire genesis file in json.
   */
  async getGenesis(requestOptions?: ApiRequestOptions): Promise<Genesis> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/genesis',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as Genesis
    try {
      return (GenesisCodecs as any).decodeJson(parsed as any) as Genesis
    } catch {}
    return parsed
  }

  /**
   * Get ledger deltas for a round.
   */
  async getLedgerStateDelta(
    round: number | bigint,
    params?: { format?: 'json' | 'msgpack' },
    requestOptions?: ApiRequestOptions,
  ): Promise<Uint8Array> {
    const headers: Record<string, string> = {}
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === 'json'
    headers['Accept'] = useJson ? 'application/json' : 'application/msgpack'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/deltas/{round}',
      path: { round: typeof round === 'bigint' ? round.toString() : round },
      query: { format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    if (params?.format === 'json') {
      const parsed = (await rsp) as Uint8Array
      try {
        return (Uint8ArrayCodecs as any).decodeJson(parsed as any) as Uint8Array
      } catch {}
      return parsed
    } else {
      const buf = (await rsp) as unknown as Uint8Array
      try {
        return (Uint8ArrayCodecs as any).decodeMsgpack(buf) as Uint8Array
      } catch {}
      return buf as unknown as Uint8Array
    }
  }

  /**
   * Get a ledger delta for a given transaction group.
   */
  async getLedgerStateDeltaForTransactionGroup(
    id: string,
    params?: { format?: 'json' | 'msgpack' },
    requestOptions?: ApiRequestOptions,
  ): Promise<Uint8Array> {
    const headers: Record<string, string> = {}
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === 'json'
    headers['Accept'] = useJson ? 'application/json' : 'application/msgpack'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/deltas/txn/group/{id}',
      path: { id: id },
      query: { format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    if (params?.format === 'json') {
      const parsed = (await rsp) as Uint8Array
      try {
        return (Uint8ArrayCodecs as any).decodeJson(parsed as any) as Uint8Array
      } catch {}
      return parsed
    } else {
      const buf = (await rsp) as unknown as Uint8Array
      try {
        return (Uint8ArrayCodecs as any).decodeMsgpack(buf) as Uint8Array
      } catch {}
      return buf as unknown as Uint8Array
    }
  }

  async getLightBlockHeaderProof(round: number | bigint, requestOptions?: ApiRequestOptions): Promise<LightBlockHeaderProof> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/blocks/{round}/lightheader/proof',
      path: { round: typeof round === 'bigint' ? round.toString() : round },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as LightBlockHeaderProof
    try {
      return (LightBlockHeaderProofCodecs as any).decodeJson(parsed as any) as LightBlockHeaderProof
    } catch {}
    return parsed
  }

  /**
   * Given a participation ID, return information about that participation key
   */
  async getParticipationKeyById(participationId: string, requestOptions?: ApiRequestOptions): Promise<ParticipationKey> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/participation/{participation-id}',
      path: { 'participation-id': participationId },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as ParticipationKey
    try {
      return (ParticipationKeyCodecs as any).decodeJson(parsed as any) as ParticipationKey
    } catch {}
    return parsed
  }

  /**
   * Return a list of participation keys
   */
  async getParticipationKeys(requestOptions?: ApiRequestOptions): Promise<ParticipationKey[]> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/participation',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as ParticipationKey[]
    try {
      return (ParticipationKeyCodecs as any).decodeJsonArray(parsed as any) as ParticipationKey[]
    } catch {}
    return parsed
  }

  /**
   * Get the list of pending transactions, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions.
   */
  async getPendingTransactions(
    params?: { max?: number | bigint; format?: 'json' | 'msgpack' },
    requestOptions?: ApiRequestOptions,
  ): Promise<Uint8Array> {
    const headers: Record<string, string> = {}
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === 'json'
    headers['Accept'] = useJson ? 'application/json' : 'application/msgpack'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/transactions/pending',
      path: {},
      query: { max: typeof params?.max === 'bigint' ? (params!.max as bigint).toString() : params?.max, format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    if (params?.format === 'json') {
      const parsed = (await rsp) as Uint8Array
      try {
        return (Uint8ArrayCodecs as any).decodeJson(parsed as any) as Uint8Array
      } catch {}
      return parsed
    } else {
      const buf = (await rsp) as unknown as Uint8Array
      try {
        return (Uint8ArrayCodecs as any).decodeMsgpack(buf) as Uint8Array
      } catch {}
      return buf as unknown as Uint8Array
    }
  }

  /**
   * Get the list of pending transactions by address, sorted by priority, in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions.
   */
  async getPendingTransactionsByAddress(
    address: string,
    params?: { max?: number | bigint; format?: 'json' | 'msgpack' },
    requestOptions?: ApiRequestOptions,
  ): Promise<Uint8Array> {
    const headers: Record<string, string> = {}
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === 'json'
    headers['Accept'] = useJson ? 'application/json' : 'application/msgpack'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/accounts/{address}/transactions/pending',
      path: { address: address },
      query: { max: typeof params?.max === 'bigint' ? (params!.max as bigint).toString() : params?.max, format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    if (params?.format === 'json') {
      const parsed = (await rsp) as Uint8Array
      try {
        return (Uint8ArrayCodecs as any).decodeJson(parsed as any) as Uint8Array
      } catch {}
      return parsed
    } else {
      const buf = (await rsp) as unknown as Uint8Array
      try {
        return (Uint8ArrayCodecs as any).decodeMsgpack(buf) as Uint8Array
      } catch {}
      return buf as unknown as Uint8Array
    }
  }

  async getReady(requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/ready',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as void
    try {
      return (voidCodecs as any).decodeJson(parsed as any) as void
    } catch {}
    return parsed
  }

  async getStateProof(round: number | bigint, requestOptions?: ApiRequestOptions): Promise<StateProof> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/stateproofs/{round}',
      path: { round: typeof round === 'bigint' ? round.toString() : round },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as StateProof
    try {
      return (StateProofCodecs as any).decodeJson(parsed as any) as StateProof
    } catch {}
    return parsed
  }

  async getStatus(requestOptions?: ApiRequestOptions): Promise<GetStatus> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/status',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as GetStatus
    try {
      return (GetStatusCodecs as any).decodeJson(parsed as any) as GetStatus
    } catch {}
    return parsed
  }

  async getSupply(requestOptions?: ApiRequestOptions): Promise<GetSupply> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/ledger/supply',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as GetSupply
    try {
      return (GetSupplyCodecs as any).decodeJson(parsed as any) as GetSupply
    } catch {}
    return parsed
  }

  /**
   * Gets the minimum sync round for the ledger.
   */
  async getSyncRound(requestOptions?: ApiRequestOptions): Promise<GetSyncRound> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/ledger/sync',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as GetSyncRound
    try {
      return (GetSyncRoundCodecs as any).decodeJson(parsed as any) as GetSyncRound
    } catch {}
    return parsed
  }

  /**
   * Get ledger deltas for transaction groups in a given round.
   */
  async getTransactionGroupLedgerStateDeltasForRound(
    round: number | bigint,
    params?: { format?: 'json' | 'msgpack' },
    requestOptions?: ApiRequestOptions,
  ): Promise<Uint8Array> {
    const headers: Record<string, string> = {}
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === 'json'
    headers['Accept'] = useJson ? 'application/json' : 'application/msgpack'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/deltas/{round}/txn/group',
      path: { round: typeof round === 'bigint' ? round.toString() : round },
      query: { format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    if (params?.format === 'json') {
      const parsed = (await rsp) as Uint8Array
      try {
        return (Uint8ArrayCodecs as any).decodeJson(parsed as any) as Uint8Array
      } catch {}
      return parsed
    } else {
      const buf = (await rsp) as unknown as Uint8Array
      try {
        return (Uint8ArrayCodecs as any).decodeMsgpack(buf) as Uint8Array
      } catch {}
      return buf as unknown as Uint8Array
    }
  }

  async getTransactionProof(
    round: number | bigint,
    txid: string,
    params?: { hashtype?: 'sha512_256' | 'sha256'; format?: 'json' | 'msgpack' },
    requestOptions?: ApiRequestOptions,
  ): Promise<TransactionProof> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/blocks/{round}/transactions/{txid}/proof',
      path: { round: typeof round === 'bigint' ? round.toString() : round, txid: txid },
      query: { hashtype: params?.hashtype, format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as TransactionProof
    try {
      return (TransactionProofCodecs as any).decodeJson(parsed as any) as TransactionProof
    } catch {}
    return parsed
  }

  /**
   * Retrieves the supported API versions, binary build versions, and genesis information.
   */
  async getVersion(requestOptions?: ApiRequestOptions): Promise<Version> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/versions',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as Version
    try {
      return (VersionCodecs as any).decodeJson(parsed as any) as Version
    } catch {}
    return parsed
  }

  async healthCheck(requestOptions?: ApiRequestOptions): Promise<void> {
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
    const parsed = (await rsp) as void
    try {
      return (voidCodecs as any).decodeJson(parsed as any) as void
    } catch {}
    return parsed
  }

  async metrics(requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/metrics',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as void
    try {
      return (voidCodecs as any).decodeJson(parsed as any) as void
    } catch {}
    return parsed
  }

  /**
   * Given a transaction ID of a recently submitted transaction, it returns information about it.  There are several cases when this might succeed:
   * - transaction committed (committed round > 0)
   * - transaction still in the pool (committed round = 0, pool error = "")
   * - transaction removed from pool due to error (committed round = 0, pool error != "")
   * Or the transaction may have happened sufficiently long ago that the node no longer remembers it, and this will return an error.
   */
  async pendingTransactionInformation(
    txid: string,
    params?: { format?: 'json' | 'msgpack' },
    requestOptions?: ApiRequestOptions,
  ): Promise<Uint8Array> {
    const headers: Record<string, string> = {}
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === 'json'
    headers['Accept'] = useJson ? 'application/json' : 'application/msgpack'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/transactions/pending/{txid}',
      path: { txid: txid },
      query: { format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    if (params?.format === 'json') {
      const parsed = (await rsp) as Uint8Array
      try {
        return (Uint8ArrayCodecs as any).decodeJson(parsed as any) as Uint8Array
      } catch {}
      return parsed
    } else {
      const buf = (await rsp) as unknown as Uint8Array
      try {
        return (Uint8ArrayCodecs as any).decodeMsgpack(buf) as Uint8Array
      } catch {}
      return buf as unknown as Uint8Array
    }
  }

  /**
   * Enables blocking and mutex profiles, and returns the old settings
   */
  async putDebugSettingsProf(requestOptions?: ApiRequestOptions): Promise<DebugSettingsProf> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'PUT',
      url: '/debug/settings/pprof',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as DebugSettingsProf
    try {
      return (DebugSettingsProfCodecs as any).decodeJson(parsed as any) as DebugSettingsProf
    } catch {}
    return parsed
  }

  async rawTransaction(params?: { body: Uint8Array }, requestOptions?: ApiRequestOptions): Promise<RawTransaction> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'
    headers['Content-Type'] = 'application/x-binary'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v2/transactions',
      path: {},
      query: {},
      headers,
      body: params?.body,
      mediaType: 'application/x-binary',
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as RawTransaction
    try {
      return (RawTransactionCodecs as any).decodeJson(parsed as any) as RawTransaction
    } catch {}
    return parsed
  }

  async rawTransactionAsync(params?: { body: Uint8Array }, requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'
    headers['Content-Type'] = 'application/x-binary'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v2/transactions/async',
      path: {},
      query: {},
      headers,
      body: params?.body,
      mediaType: 'application/x-binary',
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as void
    try {
      return (voidCodecs as any).decodeJson(parsed as any) as void
    } catch {}
    return parsed
  }

  /**
   * Sets the timestamp offset (seconds) for blocks in dev mode. Providing an offset of 0 will unset this value and try to use the real clock for the timestamp.
   */
  async setBlockTimeStampOffset(offset: number | bigint, requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v2/devmode/blocks/offset/{offset}',
      path: { offset: typeof offset === 'bigint' ? offset.toString() : offset },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as void
    try {
      return (voidCodecs as any).decodeJson(parsed as any) as void
    } catch {}
    return parsed
  }

  /**
   * Sets the minimum sync round on the ledger.
   */
  async setSyncRound(round: number | bigint, requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v2/ledger/sync/{round}',
      path: { round: typeof round === 'bigint' ? round.toString() : round },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as void
    try {
      return (voidCodecs as any).decodeJson(parsed as any) as void
    } catch {}
    return parsed
  }

  /**
   * Special management endpoint to shutdown the node. Optionally provide a timeout parameter to indicate that the node should begin shutting down after a number of seconds.
   */
  async shutdownNode(params?: { timeout?: number | bigint }, requestOptions?: ApiRequestOptions): Promise<ShutdownNode> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v2/shutdown',
      path: {},
      query: { timeout: typeof params?.timeout === 'bigint' ? (params!.timeout as bigint).toString() : params?.timeout },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as ShutdownNode
    try {
      return (ShutdownNodeCodecs as any).decodeJson(parsed as any) as ShutdownNode
    } catch {}
    return parsed
  }

  async simulateTransaction(
    params?: { format?: 'json' | 'msgpack'; body: SimulateRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<SimulateTransaction> {
    const headers: Record<string, string> = {}
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === 'json'
    headers['Accept'] = useJson ? 'application/json' : 'application/msgpack'
    headers['Content-Type'] = params?.format === 'json' ? 'application/json' : 'application/msgpack'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v2/transactions/simulate',
      path: {},
      query: { format: params?.format },
      headers,
      body: ((): any => {
        // eslint-disable-line @typescript-eslint/no-explicit-any
        const b = params?.body as any
        if (b == null) return undefined
        if (params?.format === 'json') {
          const t = 'SimulateRequest'
          if (t === 'SignedTransaction') return b // JSON not supported for raw STX
          try {
            return (SimulateRequestCodecs as any).encodeJson(b)
          } catch {}
          return b
        } else {
          const t = 'SimulateRequest'
          try {
            return (SimulateRequestCodecs as any).encodeMsgpack(b)
          } catch {}
          return b instanceof Uint8Array ? b : b
        }
      })(),
      // Dynamic mediaType based on format parameter (prefer msgpack by default)
      mediaType: params?.format === 'json' ? 'application/json' : 'application/msgpack',
      ...(requestOptions ?? {}),
    })
    if (params?.format === 'json') {
      const parsed = (await rsp) as SimulateTransaction
      try {
        return (SimulateTransactionCodecs as any).decodeJson(parsed as any) as SimulateTransaction
      } catch {}
      return parsed
    } else {
      const buf = (await rsp) as unknown as Uint8Array
      try {
        return (SimulateTransactionCodecs as any).decodeMsgpack(buf) as SimulateTransaction
      } catch {}
      return buf as unknown as SimulateTransaction
    }
  }

  /**
   * Given a catchpoint, it starts catching up to this catchpoint
   */
  async startCatchup(catchpoint: string, params?: { min?: number | bigint }, requestOptions?: ApiRequestOptions): Promise<StartCatchup> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v2/catchup/{catchpoint}',
      path: { catchpoint: catchpoint },
      query: { min: typeof params?.min === 'bigint' ? (params!.min as bigint).toString() : params?.min },
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as StartCatchup
    try {
      return (StartCatchupCodecs as any).decodeJson(parsed as any) as StartCatchup
    } catch {}
    return parsed
  }

  /**
   * Returns the entire swagger spec in json.
   */
  async swaggerJson(requestOptions?: ApiRequestOptions): Promise<string> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/swagger.json',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as string
    try {
      return (stringCodecs as any).decodeJson(parsed as any) as string
    } catch {}
    return parsed
  }

  /**
   * Given TEAL source code in plain text, return base64 encoded program bytes and base32 SHA512_256 hash of program bytes (Address style). This endpoint is only enabled when a node's configuration file sets EnableDeveloperAPI to true.
   */
  async tealCompile(params?: { sourcemap?: boolean; body: string }, requestOptions?: ApiRequestOptions): Promise<TealCompile> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'
    headers['Content-Type'] = 'text/plain'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v2/teal/compile',
      path: {},
      query: { sourcemap: params?.sourcemap },
      headers,
      body: params?.body,
      mediaType: 'text/plain',
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as TealCompile
    try {
      return (TealCompileCodecs as any).decodeJson(parsed as any) as TealCompile
    } catch {}
    return parsed
  }

  /**
   * Given the program bytes, return the TEAL source code in plain text. This endpoint is only enabled when a node's configuration file sets EnableDeveloperAPI to true.
   */
  async tealDisassemble(params?: { body: Uint8Array }, requestOptions?: ApiRequestOptions): Promise<TealDisassemble> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'
    headers['Content-Type'] = 'application/x-binary'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v2/teal/disassemble',
      path: {},
      query: {},
      headers,
      body: params?.body,
      mediaType: 'application/x-binary',
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as TealDisassemble
    try {
      return (TealDisassembleCodecs as any).decodeJson(parsed as any) as TealDisassemble
    } catch {}
    return parsed
  }

  /**
   * Executes TEAL program(s) in context and returns debugging information about the execution. This endpoint is only enabled when a node's configuration file sets EnableDeveloperAPI to true.
   */
  async tealDryrun(params?: { body?: DryrunRequest }, requestOptions?: ApiRequestOptions): Promise<TealDryrun> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'
    headers['Content-Type'] = 'application/msgpack'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v2/teal/dryrun',
      path: {},
      query: {},
      headers,
      body: ((): any => {
        const b = params?.body as any
        if (b == null) return undefined
        try {
          return (DryrunRequestCodecs as any).encodeJson(b)
        } catch {}
        return b
      })(),
      // Both supported, prefer msgpack for better performance
      mediaType: 'application/msgpack',
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as TealDryrun
    try {
      return (TealDryrunCodecs as any).decodeJson(parsed as any) as TealDryrun
    } catch {}
    return parsed
  }

  async transactionParams(requestOptions?: ApiRequestOptions): Promise<TransactionParams> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/transactions/params',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as TransactionParams
    try {
      return (TransactionParamsCodecs as any).decodeJson(parsed as any) as TransactionParams
    } catch {}
    return parsed
  }

  /**
   * Unset the ledger sync round.
   */
  async unsetSyncRound(requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'DELETE',
      url: '/v2/ledger/sync',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as void
    try {
      return (voidCodecs as any).decodeJson(parsed as any) as void
    } catch {}
    return parsed
  }

  /**
   * Waits for a block to appear after round {round} and returns the node's status at the time. There is a 1 minute timeout, when reached the current status is returned regardless of whether or not it is the round after the given round.
   */
  async waitForBlock(round: number | bigint, requestOptions?: ApiRequestOptions): Promise<WaitForBlock> {
    const headers: Record<string, string> = {}
    headers['Accept'] = 'application/json'

    // Header parameters

    const rsp = this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v2/status/wait-for-block-after/{round}',
      path: { round: typeof round === 'bigint' ? round.toString() : round },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })
    const parsed = (await rsp) as WaitForBlock
    try {
      return (WaitForBlockCodecs as any).decodeJson(parsed as any) as WaitForBlock
    } catch {}
    return parsed
  }
}
