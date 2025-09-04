import type { BaseHttpRequest, ApiRequestOptions } from "../core/BaseHttpRequest";
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
} from "../models/index";

export class ApiService {
  constructor(public readonly httpRequest: BaseHttpRequest) {}

  lookupAccountAppLocalStates(
    accountId: string,
    params?: { applicationId?: number; includeAll?: boolean; limit?: number; next?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAccountAppLocalStates> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/accounts/{account-id}/apps-local-state",
      path: { "account-id": accountId },
      query: { "application-id": params?.applicationId, "include-all": params?.includeAll, limit: params?.limit, next: params?.next },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  lookupAccountAssets(
    accountId: string,
    params?: { assetId?: number; includeAll?: boolean; limit?: number; next?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAccountAssets> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/accounts/{account-id}/assets",
      path: { "account-id": accountId },
      query: { "asset-id": params?.assetId, "include-all": params?.includeAll, limit: params?.limit, next: params?.next },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  lookupAccountById(
    accountId: string,
    params?: {
      round?: number;
      includeAll?: boolean;
      exclude?: "all" | "assets" | "created-assets" | "apps-local-state" | "created-apps" | "none"[];
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAccountById> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/accounts/{account-id}",
      path: { "account-id": accountId },
      query: { round: params?.round, "include-all": params?.includeAll, exclude: params?.exclude },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  lookupAccountCreatedApplications(
    accountId: string,
    params?: { applicationId?: number; includeAll?: boolean; limit?: number; next?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAccountCreatedApplications> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/accounts/{account-id}/created-applications",
      path: { "account-id": accountId },
      query: { "application-id": params?.applicationId, "include-all": params?.includeAll, limit: params?.limit, next: params?.next },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  lookupAccountCreatedAssets(
    accountId: string,
    params?: { assetId?: number; includeAll?: boolean; limit?: number; next?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAccountCreatedAssets> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/accounts/{account-id}/created-assets",
      path: { "account-id": accountId },
      query: { "asset-id": params?.assetId, "include-all": params?.includeAll, limit: params?.limit, next: params?.next },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  lookupAccountTransactions(
    accountId: string,
    params?: {
      limit?: number;
      next?: string;
      notePrefix?: string;
      txType?: "pay" | "keyreg" | "acfg" | "axfer" | "afrz" | "appl" | "stpf" | "hb";
      sigType?: "sig" | "msig" | "lsig";
      txid?: string;
      round?: number;
      minRound?: number;
      maxRound?: number;
      assetId?: number;
      beforeTime?: string;
      afterTime?: string;
      currencyGreaterThan?: number;
      currencyLessThan?: number;
      rekeyTo?: boolean;
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAccountTransactions> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/accounts/{account-id}/transactions",
      path: { "account-id": accountId },
      query: {
        limit: params?.limit,
        next: params?.next,
        "note-prefix": params?.notePrefix,
        "tx-type": params?.txType,
        "sig-type": params?.sigType,
        txid: params?.txid,
        round: params?.round,
        "min-round": params?.minRound,
        "max-round": params?.maxRound,
        "asset-id": params?.assetId,
        "before-time": params?.beforeTime,
        "after-time": params?.afterTime,
        "currency-greater-than": params?.currencyGreaterThan,
        "currency-less-than": params?.currencyLessThan,
        "rekey-to": params?.rekeyTo,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  lookupApplicationBoxByIdandName(applicationId: number, params?: { name: string }, requestOptions?: ApiRequestOptions): Promise<Box> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/applications/{application-id}/box",
      path: { "application-id": applicationId },
      query: { name: params?.name },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  lookupApplicationById(
    applicationId: number,
    params?: { includeAll?: boolean },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupApplicationById> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/applications/{application-id}",
      path: { "application-id": applicationId },
      query: { "include-all": params?.includeAll },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  lookupApplicationLogsById(
    applicationId: number,
    params?: { limit?: number; next?: string; txid?: string; minRound?: number; maxRound?: number; senderAddress?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupApplicationLogsById> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/applications/{application-id}/logs",
      path: { "application-id": applicationId },
      query: {
        limit: params?.limit,
        next: params?.next,
        txid: params?.txid,
        "min-round": params?.minRound,
        "max-round": params?.maxRound,
        "sender-address": params?.senderAddress,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  lookupAssetBalances(
    assetId: number,
    params?: { includeAll?: boolean; limit?: number; next?: string; currencyGreaterThan?: number; currencyLessThan?: number },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAssetBalances> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/assets/{asset-id}/balances",
      path: { "asset-id": assetId },
      query: {
        "include-all": params?.includeAll,
        limit: params?.limit,
        next: params?.next,
        "currency-greater-than": params?.currencyGreaterThan,
        "currency-less-than": params?.currencyLessThan,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  lookupAssetById(assetId: number, params?: { includeAll?: boolean }, requestOptions?: ApiRequestOptions): Promise<LookupAssetById> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/assets/{asset-id}",
      path: { "asset-id": assetId },
      query: { "include-all": params?.includeAll },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  lookupAssetTransactions(
    assetId: number,
    params?: {
      limit?: number;
      next?: string;
      notePrefix?: string;
      txType?: "pay" | "keyreg" | "acfg" | "axfer" | "afrz" | "appl" | "stpf" | "hb";
      sigType?: "sig" | "msig" | "lsig";
      txid?: string;
      round?: number;
      minRound?: number;
      maxRound?: number;
      beforeTime?: string;
      afterTime?: string;
      currencyGreaterThan?: number;
      currencyLessThan?: number;
      address?: string;
      addressRole?: "sender" | "receiver" | "freeze-target";
      excludeCloseTo?: boolean;
      rekeyTo?: boolean;
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<LookupAssetTransactions> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/assets/{asset-id}/transactions",
      path: { "asset-id": assetId },
      query: {
        limit: params?.limit,
        next: params?.next,
        "note-prefix": params?.notePrefix,
        "tx-type": params?.txType,
        "sig-type": params?.sigType,
        txid: params?.txid,
        round: params?.round,
        "min-round": params?.minRound,
        "max-round": params?.maxRound,
        "before-time": params?.beforeTime,
        "after-time": params?.afterTime,
        "currency-greater-than": params?.currencyGreaterThan,
        "currency-less-than": params?.currencyLessThan,
        address: params?.address,
        "address-role": params?.addressRole,
        "exclude-close-to": params?.excludeCloseTo,
        "rekey-to": params?.rekeyTo,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  lookupBlock(roundNumber: number, params?: { headerOnly?: boolean }, requestOptions?: ApiRequestOptions): Promise<Block> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/blocks/{round-number}",
      path: { "round-number": roundNumber },
      query: { "header-only": params?.headerOnly },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  lookupTransaction(txid: string, requestOptions?: ApiRequestOptions): Promise<LookupTransaction> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/transactions/{txid}",
      path: { txid: txid },
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  makeHealthCheck(requestOptions?: ApiRequestOptions): Promise<HealthCheck> {
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

  searchForAccounts(
    params?: {
      assetId?: number;
      limit?: number;
      next?: string;
      currencyGreaterThan?: number;
      includeAll?: boolean;
      exclude?: "all" | "assets" | "created-assets" | "apps-local-state" | "created-apps" | "none"[];
      currencyLessThan?: number;
      authAddr?: string;
      round?: number;
      applicationId?: number;
      onlineOnly?: boolean;
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<SearchForAccounts> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/accounts",
      path: {},
      query: {
        "asset-id": params?.assetId,
        limit: params?.limit,
        next: params?.next,
        "currency-greater-than": params?.currencyGreaterThan,
        "include-all": params?.includeAll,
        exclude: params?.exclude,
        "currency-less-than": params?.currencyLessThan,
        "auth-addr": params?.authAddr,
        round: params?.round,
        "application-id": params?.applicationId,
        "online-only": params?.onlineOnly,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  searchForApplicationBoxes(
    applicationId: number,
    params?: { limit?: number; next?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<SearchForApplicationBoxes> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/applications/{application-id}/boxes",
      path: { "application-id": applicationId },
      query: { limit: params?.limit, next: params?.next },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  searchForApplications(
    params?: { applicationId?: number; creator?: string; includeAll?: boolean; limit?: number; next?: string },
    requestOptions?: ApiRequestOptions,
  ): Promise<SearchForApplications> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/applications",
      path: {},
      query: {
        "application-id": params?.applicationId,
        creator: params?.creator,
        "include-all": params?.includeAll,
        limit: params?.limit,
        next: params?.next,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  searchForAssets(
    params?: { includeAll?: boolean; limit?: number; next?: string; creator?: string; name?: string; unit?: string; assetId?: number },
    requestOptions?: ApiRequestOptions,
  ): Promise<SearchForAssets> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/assets",
      path: {},
      query: {
        "include-all": params?.includeAll,
        limit: params?.limit,
        next: params?.next,
        creator: params?.creator,
        name: params?.name,
        unit: params?.unit,
        "asset-id": params?.assetId,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  searchForBlockHeaders(
    params?: {
      limit?: number;
      next?: string;
      minRound?: number;
      maxRound?: number;
      beforeTime?: string;
      afterTime?: string;
      proposers?: string[];
      expired?: string[];
      absent?: string[];
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<SearchForBlockHeaders> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/block-headers",
      path: {},
      query: {
        limit: params?.limit,
        next: params?.next,
        "min-round": params?.minRound,
        "max-round": params?.maxRound,
        "before-time": params?.beforeTime,
        "after-time": params?.afterTime,
        proposers: params?.proposers,
        expired: params?.expired,
        absent: params?.absent,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }

  searchForTransactions(
    params?: {
      limit?: number;
      next?: string;
      notePrefix?: string;
      txType?: "pay" | "keyreg" | "acfg" | "axfer" | "afrz" | "appl" | "stpf" | "hb";
      sigType?: "sig" | "msig" | "lsig";
      groupId?: string;
      txid?: string;
      round?: number;
      minRound?: number;
      maxRound?: number;
      assetId?: number;
      beforeTime?: string;
      afterTime?: string;
      currencyGreaterThan?: number;
      currencyLessThan?: number;
      address?: string;
      addressRole?: "sender" | "receiver" | "freeze-target";
      excludeCloseTo?: boolean;
      rekeyTo?: boolean;
      applicationId?: number;
    },
    requestOptions?: ApiRequestOptions,
  ): Promise<SearchForTransactions> {
    const headers: Record<string, string> = {};
    headers["Accept"] = "application/json";

    // Header parameters

    return this.httpRequest.request({
      method: "GET",
      url: "/v2/transactions",
      path: {},
      query: {
        limit: params?.limit,
        next: params?.next,
        "note-prefix": params?.notePrefix,
        "tx-type": params?.txType,
        "sig-type": params?.sigType,
        "group-id": params?.groupId,
        txid: params?.txid,
        round: params?.round,
        "min-round": params?.minRound,
        "max-round": params?.maxRound,
        "asset-id": params?.assetId,
        "before-time": params?.beforeTime,
        "after-time": params?.afterTime,
        "currency-greater-than": params?.currencyGreaterThan,
        "currency-less-than": params?.currencyLessThan,
        address: params?.address,
        "address-role": params?.addressRole,
        "exclude-close-to": params?.excludeCloseTo,
        "rekey-to": params?.rekeyTo,
        "application-id": params?.applicationId,
      },
      headers,
      body: undefined,
      mediaType: undefined,
      expectBinary: false,
      ...(requestOptions ?? {}),
    });
  }
}
