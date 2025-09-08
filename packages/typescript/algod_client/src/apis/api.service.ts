import type { BaseHttpRequest, ApiRequestOptions } from "../core/BaseHttpRequest";
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
  GetBlock,
  GetBlockHash,
  GetBlockLogs,
  GetBlockTimeStampOffset,
  GetBlockTxids,
  GetPendingTransactions,
  GetPendingTransactionsByAddress,
  GetStatus,
  GetSupply,
  GetSyncRound,
  GetTransactionGroupLedgerStateDeltasForRound,
  LedgerStateDelta,
  LightBlockHeaderProof,
  ParticipationKey,
  PendingTransactionResponse,
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
} from "../models/index";

export class AlgodApi {
  constructor(public readonly httpRequest: BaseHttpRequest) {}

  abortCatchup(catchpoint: string, requestOptions?: ApiRequestOptions): Promise<AbortCatchup> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "DELETE",
      url: "/v2/catchup/{catchpoint}",
      path: { catchpoint: catchpoint },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  accountApplicationInformation(
    address: string,
    applicationId: number | bigint,
    params?: { format?: "json" | "msgpack" },
    requestOptions?: ApiRequestOptions,
  ): Promise<AccountApplicationInformation> {
    const headers: Record<string, string> = {};
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === "json";
    headers["Accept"] = useJson ? "application/json" : "application/msgpack";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/accounts/{address}/applications/{application-id}",
      path: { address: address, "application-id": typeof applicationId === "bigint" ? applicationId.toString() : applicationId },
      query: { format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: params?.format === "json" ? false : true,
      ...(requestOptions ?? {}),
    });
  }

  accountAssetInformation(
    address: string,
    assetId: number | bigint,
    params?: { format?: "json" | "msgpack" },
    requestOptions?: ApiRequestOptions,
  ): Promise<AccountAssetInformation> {
    const headers: Record<string, string> = {};
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === "json";
    headers["Accept"] = useJson ? "application/json" : "application/msgpack";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/accounts/{address}/assets/{asset-id}",
      path: { address: address, "asset-id": typeof assetId === "bigint" ? assetId.toString() : assetId },
      query: { format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: params?.format === "json" ? false : true,
      ...(requestOptions ?? {}),
    });
  }

  accountAssetsInformation(
    address: string,
    params?: { limit?: number | bigint; next?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<AccountAssetsInformation> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/accounts/{address}/assets",
      path: { address: address },
      query: { limit: typeof params?.limit === "bigint" ? (params!.limit as bigint).toString() : params?.limit, next: params?.next },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  accountInformation(
    address: string,
    params?: { exclude?: "all" | "none"; format?: "json" | "msgpack" },
    requestOptions?: ApiRequestOptions,
  ): Promise<Account> {
    const headers: Record<string, string> = {};
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === "json";
    headers["Accept"] = useJson ? "application/json" : "application/msgpack";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/accounts/{address}",
      path: { address: address },
      query: { exclude: params?.exclude, format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: params?.format === "json" ? false : true,
      ...(requestOptions ?? {}),
    });
  }

  addParticipationKey(params?: { body: string }, requestOptions?: ApiRequestOptions): Promise<AddParticipationKey> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";
    headers["Content-Type"] = "application/msgpack";

    // Header parameters

    return this.httpRequest.request({
      method: "POST",
      url: "/v2/participation",
      path: {},
      query: {},
      headers,
      body: params?.body,
      // Only msgpack supported for request body
      mediaType: "application/msgpack",
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  appendKeys(participationId: string, params?: { body: string }, requestOptions?: ApiRequestOptions): Promise<ParticipationKey> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";
    headers["Content-Type"] = "application/msgpack";

    // Header parameters

    return this.httpRequest.request({
      method: "POST",
      url: "/v2/participation/{participation-id}",
      path: { "participation-id": participationId },
      query: {},
      headers,
      body: params?.body,
      // Only msgpack supported for request body
      mediaType: "application/msgpack",
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  deleteParticipationKeyById(participationId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "DELETE",
      url: "/v2/participation/{participation-id}",
      path: { "participation-id": participationId },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  experimentalCheck(requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/experimental",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  generateParticipationKeys(
    address: string,
    params?: { dilution?: number | bigint; first: number | bigint; last: number | bigint },
    requestOptions?: ApiRequestOptions,
  ): Promise<string> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "POST",
      url: "/v2/participation/generate/{address}",
      path: { address: address },
      query: {
        dilution: typeof params?.dilution === "bigint" ? (params!.dilution as bigint).toString() : params?.dilution,
        first: typeof params?.first === "bigint" ? (params!.first as bigint).toString() : params?.first,
        last: typeof params?.last === "bigint" ? (params!.last as bigint).toString() : params?.last,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getApplicationBoxByName(applicationId: number | bigint, params?: { name: string }, requestOptions?: ApiRequestOptions): Promise<Box> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/applications/{application-id}/box",
      path: { "application-id": typeof applicationId === "bigint" ? applicationId.toString() : applicationId },
      query: { name: params?.name },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getApplicationBoxes(
    applicationId: number | bigint,
    params?: { max?: number | bigint },
    requestOptions?: ApiRequestOptions,
  ): Promise<GetApplicationBoxes> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/applications/{application-id}/boxes",
      path: { "application-id": typeof applicationId === "bigint" ? applicationId.toString() : applicationId },
      query: { max: typeof params?.max === "bigint" ? (params!.max as bigint).toString() : params?.max },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getApplicationById(applicationId: number | bigint, requestOptions?: ApiRequestOptions): Promise<Application> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/applications/{application-id}",
      path: { "application-id": typeof applicationId === "bigint" ? applicationId.toString() : applicationId },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getAssetById(assetId: number | bigint, requestOptions?: ApiRequestOptions): Promise<Asset> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/assets/{asset-id}",
      path: { "asset-id": typeof assetId === "bigint" ? assetId.toString() : assetId },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getBlock(
    round: number | bigint,
    params?: { headerOnly?: boolean; format?: "json" | "msgpack" },
    requestOptions?: ApiRequestOptions,
  ): Promise<GetBlock> {
    const headers: Record<string, string> = {};
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === "json";
    headers["Accept"] = useJson ? "application/json" : "application/msgpack";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/blocks/{round}",
      path: { round: typeof round === "bigint" ? round.toString() : round },
      query: { "header-only": params?.headerOnly, format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: params?.format === "json" ? false : true,
      ...(requestOptions ?? {}),
    });
  }

  getBlockHash(round: number | bigint, requestOptions?: ApiRequestOptions): Promise<GetBlockHash> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/blocks/{round}/hash",
      path: { round: typeof round === "bigint" ? round.toString() : round },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getBlockLogs(round: number | bigint, requestOptions?: ApiRequestOptions): Promise<GetBlockLogs> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/blocks/{round}/logs",
      path: { round: typeof round === "bigint" ? round.toString() : round },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getBlockTimeStampOffset(requestOptions?: ApiRequestOptions): Promise<GetBlockTimeStampOffset> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/devmode/blocks/offset",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getBlockTxids(round: number | bigint, requestOptions?: ApiRequestOptions): Promise<GetBlockTxids> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/blocks/{round}/txids",
      path: { round: typeof round === "bigint" ? round.toString() : round },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getConfig(requestOptions?: ApiRequestOptions): Promise<string> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/debug/settings/config",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getDebugSettingsProf(requestOptions?: ApiRequestOptions): Promise<DebugSettingsProf> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/debug/settings/pprof",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getGenesis(requestOptions?: ApiRequestOptions): Promise<Genesis> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/genesis",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getLedgerStateDelta(
    round: number | bigint,
    params?: { format?: "json" | "msgpack" },
    requestOptions?: ApiRequestOptions,
  ): Promise<LedgerStateDelta> {
    const headers: Record<string, string> = {};
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === "json";
    headers["Accept"] = useJson ? "application/json" : "application/msgpack";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/deltas/{round}",
      path: { round: typeof round === "bigint" ? round.toString() : round },
      query: { format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: params?.format === "json" ? false : true,
      ...(requestOptions ?? {}),
    });
  }

  getLedgerStateDeltaForTransactionGroup(
    id: string,
    params?: { format?: "json" | "msgpack" },
    requestOptions?: ApiRequestOptions,
  ): Promise<LedgerStateDelta> {
    const headers: Record<string, string> = {};
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === "json";
    headers["Accept"] = useJson ? "application/json" : "application/msgpack";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/deltas/txn/group/{id}",
      path: { id: id },
      query: { format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: params?.format === "json" ? false : true,
      ...(requestOptions ?? {}),
    });
  }

  getLightBlockHeaderProof(round: number | bigint, requestOptions?: ApiRequestOptions): Promise<LightBlockHeaderProof> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/blocks/{round}/lightheader/proof",
      path: { round: typeof round === "bigint" ? round.toString() : round },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getParticipationKeyById(participationId: string, requestOptions?: ApiRequestOptions): Promise<ParticipationKey> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/participation/{participation-id}",
      path: { "participation-id": participationId },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getParticipationKeys(requestOptions?: ApiRequestOptions): Promise<ParticipationKey[]> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/participation",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getPendingTransactions(
    params?: { max?: number | bigint; format?: "json" | "msgpack" },
    requestOptions?: ApiRequestOptions,
  ): Promise<GetPendingTransactions> {
    const headers: Record<string, string> = {};
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === "json";
    headers["Accept"] = useJson ? "application/json" : "application/msgpack";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/transactions/pending",
      path: {},
      query: { max: typeof params?.max === "bigint" ? (params!.max as bigint).toString() : params?.max, format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: params?.format === "json" ? false : true,
      ...(requestOptions ?? {}),
    });
  }

  getPendingTransactionsByAddress(
    address: string,
    params?: { max?: number | bigint; format?: "json" | "msgpack" },
    requestOptions?: ApiRequestOptions,
  ): Promise<GetPendingTransactionsByAddress> {
    const headers: Record<string, string> = {};
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === "json";
    headers["Accept"] = useJson ? "application/json" : "application/msgpack";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/accounts/{address}/transactions/pending",
      path: { address: address },
      query: { max: typeof params?.max === "bigint" ? (params!.max as bigint).toString() : params?.max, format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: params?.format === "json" ? false : true,
      ...(requestOptions ?? {}),
    });
  }

  getReady(requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/ready",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getStateProof(round: number | bigint, requestOptions?: ApiRequestOptions): Promise<StateProof> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/stateproofs/{round}",
      path: { round: typeof round === "bigint" ? round.toString() : round },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getStatus(requestOptions?: ApiRequestOptions): Promise<GetStatus> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/status",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getSupply(requestOptions?: ApiRequestOptions): Promise<GetSupply> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/ledger/supply",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getSyncRound(requestOptions?: ApiRequestOptions): Promise<GetSyncRound> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/ledger/sync",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getTransactionGroupLedgerStateDeltasForRound(
    round: number | bigint,
    params?: { format?: "json" | "msgpack" },
    requestOptions?: ApiRequestOptions,
  ): Promise<GetTransactionGroupLedgerStateDeltasForRound> {
    const headers: Record<string, string> = {};
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === "json";
    headers["Accept"] = useJson ? "application/json" : "application/msgpack";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/deltas/{round}/txn/group",
      path: { round: typeof round === "bigint" ? round.toString() : round },
      query: { format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: params?.format === "json" ? false : true,
      ...(requestOptions ?? {}),
    });
  }

  getTransactionProof(
    round: number | bigint,
    txid: string,
    params?: { hashtype?: "sha512_256" | "sha256"; format?: "json" | "msgpack" },
    requestOptions?: ApiRequestOptions,
  ): Promise<TransactionProof> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/blocks/{round}/transactions/{txid}/proof",
      path: { round: typeof round === "bigint" ? round.toString() : round, txid: txid },
      query: { hashtype: params?.hashtype, format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  getVersion(requestOptions?: ApiRequestOptions): Promise<Version> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/versions",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  healthCheck(requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/health",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  metrics(requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/metrics",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  pendingTransactionInformation(
    txid: string,
    params?: { format?: "json" | "msgpack" },
    requestOptions?: ApiRequestOptions,
  ): Promise<PendingTransactionResponse> {
    const headers: Record<string, string> = {};
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === "json";
    headers["Accept"] = useJson ? "application/json" : "application/msgpack";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/transactions/pending/{txid}",
      path: { txid: txid },
      query: { format: params?.format },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: params?.format === "json" ? false : true,
      ...(requestOptions ?? {}),
    });
  }

  putDebugSettingsProf(requestOptions?: ApiRequestOptions): Promise<DebugSettingsProf> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "PUT",
      url: "/debug/settings/pprof",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  rawTransaction(params?: { body: Uint8Array }, requestOptions?: ApiRequestOptions): Promise<RawTransaction> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";
    headers["Content-Type"] = "application/x-binary";

    // Header parameters

    return this.httpRequest.request({
      method: "POST",
      url: "/v2/transactions",
      path: {},
      query: {},
      headers,
      body: params?.body,
      mediaType: "application/x-binary",
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  rawTransactionAsync(params?: { body: Uint8Array }, requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";
    headers["Content-Type"] = "application/x-binary";

    // Header parameters

    return this.httpRequest.request({
      method: "POST",
      url: "/v2/transactions/async",
      path: {},
      query: {},
      headers,
      body: params?.body,
      mediaType: "application/x-binary",
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  setBlockTimeStampOffset(offset: number | bigint, requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "POST",
      url: "/v2/devmode/blocks/offset/{offset}",
      path: { offset: typeof offset === "bigint" ? offset.toString() : offset },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  setSyncRound(round: number | bigint, requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "POST",
      url: "/v2/ledger/sync/{round}",
      path: { round: typeof round === "bigint" ? round.toString() : round },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  shutdownNode(params?: { timeout?: number | bigint }, requestOptions?: ApiRequestOptions): Promise<ShutdownNode> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "POST",
      url: "/v2/shutdown",
      path: {},
      query: { timeout: typeof params?.timeout === "bigint" ? (params!.timeout as bigint).toString() : params?.timeout },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  simulateTransaction(
    params?: { format?: "json" | "msgpack"; body: SimulateRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<SimulateTransaction> {
    const headers: Record<string, string> = {};
    // Content negotiation (aligned with Rust behavior):
    // - Default to msgpack when available (better performance, smaller payload)
    // - Only use JSON if explicitly requested via format=json
    const useJson = params?.format === "json";
    headers["Accept"] = useJson ? "application/json" : "application/msgpack";
    headers["Content-Type"] = params?.format === "json" ? "application/json" : "application/msgpack";

    // Header parameters

    return this.httpRequest.request({
      method: "POST",
      url: "/v2/transactions/simulate",
      path: {},
      query: { format: params?.format },
      headers,
      body: params?.body,
      // Dynamic mediaType based on format parameter (prefer msgpack by default)
      mediaType: params?.format === "json" ? "application/json" : "application/msgpack",
      expectBinary: params?.format === "json" ? false : true,
      ...(requestOptions ?? {}),
    });
  }

  startCatchup(catchpoint: string, params?: { min?: number | bigint }, requestOptions?: ApiRequestOptions): Promise<StartCatchup> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "POST",
      url: "/v2/catchup/{catchpoint}",
      path: { catchpoint: catchpoint },
      query: { min: typeof params?.min === "bigint" ? (params!.min as bigint).toString() : params?.min },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  swaggerJson(requestOptions?: ApiRequestOptions): Promise<string> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/swagger.json",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  tealCompile(params?: { sourcemap?: boolean; body: string }, requestOptions?: ApiRequestOptions): Promise<TealCompile> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";
    headers["Content-Type"] = "text/plain";

    // Header parameters

    return this.httpRequest.request({
      method: "POST",
      url: "/v2/teal/compile",
      path: {},
      query: { sourcemap: params?.sourcemap },
      headers,
      body: params?.body,
      mediaType: "text/plain",
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  tealDisassemble(params?: { body: Uint8Array }, requestOptions?: ApiRequestOptions): Promise<TealDisassemble> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";
    headers["Content-Type"] = "application/x-binary";

    // Header parameters

    return this.httpRequest.request({
      method: "POST",
      url: "/v2/teal/disassemble",
      path: {},
      query: {},
      headers,
      body: params?.body,
      mediaType: "application/x-binary",
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  tealDryrun(params?: { body?: DryrunRequest }, requestOptions?: ApiRequestOptions): Promise<TealDryrun> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";
    headers["Content-Type"] = "application/msgpack";

    // Header parameters

    return this.httpRequest.request({
      method: "POST",
      url: "/v2/teal/dryrun",
      path: {},
      query: {},
      headers,
      body: params?.body,
      // Both supported, prefer msgpack for better performance
      mediaType: "application/msgpack",
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  transactionParams(requestOptions?: ApiRequestOptions): Promise<TransactionParams> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/transactions/params",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  unsetSyncRound(requestOptions?: ApiRequestOptions): Promise<void> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "DELETE",
      url: "/v2/ledger/sync",
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  waitForBlock(round: number | bigint, requestOptions?: ApiRequestOptions): Promise<WaitForBlock> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/status/wait-for-block-after/{round}",
      path: { round: typeof round === "bigint" ? round.toString() : round },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }
}
