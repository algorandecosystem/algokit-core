import type { BaseHttpRequest, ApiRequestOptions } from '../core/base-http-request'
import { AlgorandSerializer } from '../core/model-runtime'
import type {
  Apiv1DeleteKeyResponse,
  Apiv1DeleteMultisigResponse,
  Apiv1GetWalletsResponse,
  Apiv1PostKeyExportResponse,
  Apiv1PostKeyImportResponse,
  Apiv1PostKeyListResponse,
  Apiv1PostKeyResponse,
  Apiv1PostMasterKeyExportResponse,
  Apiv1PostMultisigExportResponse,
  Apiv1PostMultisigImportResponse,
  Apiv1PostMultisigListResponse,
  Apiv1PostMultisigProgramSignResponse,
  Apiv1PostMultisigTransactionSignResponse,
  Apiv1PostProgramSignResponse,
  Apiv1PostTransactionSignResponse,
  Apiv1PostWalletInfoResponse,
  Apiv1PostWalletInitResponse,
  Apiv1PostWalletReleaseResponse,
  Apiv1PostWalletRenameResponse,
  Apiv1PostWalletRenewResponse,
  Apiv1PostWalletResponse,
  CreateWalletRequest,
  ExportKeyRequest,
  ExportMasterKeyRequest,
  ExportMultisigRequest,
  GenerateKeyRequest,
  ImportKeyRequest,
  ImportMultisigRequest,
  InitWalletHandleTokenRequest,
  ListKeysRequest,
  ListMultisigRequest,
  ReleaseWalletHandleTokenRequest,
  RenameWalletRequest,
  RenewWalletHandleTokenRequest,
  SignMultisigRequest,
  SignProgramMultisigRequest,
  SignProgramRequest,
  SignTransactionRequest,
  VersionsResponse,
  WalletInfoRequest,
} from '../models/index'
import {
  Apiv1DeleteKeyResponseMeta,
  Apiv1DeleteMultisigResponseMeta,
  Apiv1GetWalletsResponseMeta,
  Apiv1PostKeyExportResponseMeta,
  Apiv1PostKeyImportResponseMeta,
  Apiv1PostKeyListResponseMeta,
  Apiv1PostKeyResponseMeta,
  Apiv1PostMasterKeyExportResponseMeta,
  Apiv1PostMultisigExportResponseMeta,
  Apiv1PostMultisigImportResponseMeta,
  Apiv1PostMultisigListResponseMeta,
  Apiv1PostMultisigProgramSignResponseMeta,
  Apiv1PostMultisigTransactionSignResponseMeta,
  Apiv1PostProgramSignResponseMeta,
  Apiv1PostTransactionSignResponseMeta,
  Apiv1PostWalletInfoResponseMeta,
  Apiv1PostWalletInitResponseMeta,
  Apiv1PostWalletReleaseResponseMeta,
  Apiv1PostWalletRenameResponseMeta,
  Apiv1PostWalletRenewResponseMeta,
  Apiv1PostWalletResponseMeta,
  CreateWalletRequestMeta,
  ExportKeyRequestMeta,
  ExportMasterKeyRequestMeta,
  ExportMultisigRequestMeta,
  GenerateKeyRequestMeta,
  ImportKeyRequestMeta,
  ImportMultisigRequestMeta,
  InitWalletHandleTokenRequestMeta,
  ListKeysRequestMeta,
  ListMultisigRequestMeta,
  ReleaseWalletHandleTokenRequestMeta,
  RenameWalletRequestMeta,
  RenewWalletHandleTokenRequestMeta,
  SignMultisigRequestMeta,
  SignProgramMultisigRequestMeta,
  SignProgramRequestMeta,
  SignTransactionRequestMeta,
  VersionsResponseMeta,
  WalletInfoRequestMeta,
} from '../models/index'

export class KmdApi {
  constructor(public readonly httpRequest: BaseHttpRequest) {}

  /**
   * Create a new wallet (collection of keys) with the given parameters.
   */
  async createWallet(params?: { body: CreateWalletRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostWalletResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = CreateWalletRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/wallet',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostWalletResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostWalletResponse
  }

  /**
   * Deletes the key with the passed public key from the wallet.
   */
  async deleteKey(requestOptions?: ApiRequestOptions): Promise<Apiv1DeleteKeyResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const payload = await this.httpRequest.request<unknown>({
      method: 'DELETE',
      url: '/v1/key',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1DeleteKeyResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1DeleteKeyResponse
  }

  /**
   * Deletes multisig preimage information for the passed address from the wallet.
   */
  async deleteMultisig(requestOptions?: ApiRequestOptions): Promise<Apiv1DeleteMultisigResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const payload = await this.httpRequest.request<unknown>({
      method: 'DELETE',
      url: '/v1/multisig',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1DeleteMultisigResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1DeleteMultisigResponse
  }

  /**
   * Export the secret key associated with the passed public key.
   */
  async exportKey(params?: { body: ExportKeyRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostKeyExportResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = ExportKeyRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/key/export',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostKeyExportResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostKeyExportResponse
  }

  /**
   * Export the master derivation key from the wallet. This key is a master "backup" key for the underlying wallet. With it, you can regenerate all of the wallets that have been generated with this wallet's `POST /v1/key` endpoint. This key will not allow you to recover keys imported from other wallets, however.
   */
  async exportMasterKey(
    params?: { body: ExportMasterKeyRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostMasterKeyExportResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = ExportMasterKeyRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/master-key/export',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostMasterKeyExportResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostMasterKeyExportResponse
  }

  /**
   * Given a multisig address whose preimage this wallet stores, returns the information used to generate the address, including public keys, threshold, and multisig version.
   */
  async exportMultisig(
    params?: { body: ExportMultisigRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostMultisigExportResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = ExportMultisigRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/multisig/export',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostMultisigExportResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostMultisigExportResponse
  }

  /**
   * Generates the next key in the deterministic key sequence (as determined by the master derivation key) and adds it to the wallet, returning the public key.
   */
  async generateKey(params?: { body: GenerateKeyRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostKeyResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = GenerateKeyRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/key',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostKeyResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostKeyResponse
  }

  async getVersion(requestOptions?: ApiRequestOptions): Promise<VersionsResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const payload = await this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/versions',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })

    const responseMeta = VersionsResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as VersionsResponse
  }

  /**
   * Returns information about the wallet associated with the passed wallet handle token. Additionally returns expiration information about the token itself.
   */
  async getWalletInfo(params?: { body: WalletInfoRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostWalletInfoResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = WalletInfoRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/wallet/info',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostWalletInfoResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostWalletInfoResponse
  }

  /**
   * Import an externally generated key into the wallet. Note that if you wish to back up the imported key, you must do so by backing up the entire wallet database, because imported keys were not derived from the wallet's master derivation key.
   */
  async importKey(params?: { body: ImportKeyRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostKeyImportResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = ImportKeyRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/key/import',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostKeyImportResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostKeyImportResponse
  }

  /**
   * Generates a multisig account from the passed public keys array and multisig metadata, and stores all of this in the wallet.
   */
  async importMultisig(
    params?: { body: ImportMultisigRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostMultisigImportResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = ImportMultisigRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/multisig/import',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostMultisigImportResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostMultisigImportResponse
  }

  /**
   * Unlock the wallet and return a wallet handle token that can be used for subsequent operations. These tokens expire periodically and must be renewed. You can `POST` the token to `/v1/wallet/info` to see how much time remains until expiration, and renew it with `/v1/wallet/renew`. When you're done, you can invalidate the token with `/v1/wallet/release`.
   */
  async initWalletHandleToken(
    params?: { body: InitWalletHandleTokenRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostWalletInitResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = InitWalletHandleTokenRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/wallet/init',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostWalletInitResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostWalletInitResponse
  }

  /**
   * Lists all of the public keys in this wallet. All of them have a stored private key.
   */
  async listKeysInWallet(params?: { body: ListKeysRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostKeyListResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = ListKeysRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/key/list',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostKeyListResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostKeyListResponse
  }

  /**
   * Lists all of the multisig accounts whose preimages this wallet stores
   */
  async listMultisg(params?: { body: ListMultisigRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostMultisigListResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = ListMultisigRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/multisig/list',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostMultisigListResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostMultisigListResponse
  }

  /**
   * Lists all of the wallets that kmd is aware of.
   */
  async listWallets(requestOptions?: ApiRequestOptions): Promise<Apiv1GetWalletsResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const payload = await this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v1/wallets',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1GetWalletsResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1GetWalletsResponse
  }

  /**
   * Invalidate the passed wallet handle token, making it invalid for use in subsequent requests.
   */
  async releaseWalletHandleToken(
    params?: { body: ReleaseWalletHandleTokenRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostWalletReleaseResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = ReleaseWalletHandleTokenRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/wallet/release',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostWalletReleaseResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostWalletReleaseResponse
  }

  /**
   * Rename the underlying wallet to something else
   */
  async renameWallet(params?: { body: RenameWalletRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostWalletRenameResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = RenameWalletRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/wallet/rename',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostWalletRenameResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostWalletRenameResponse
  }

  /**
   * Renew a wallet handle token, increasing its expiration duration to its initial value
   */
  async renewWalletHandleToken(
    params?: { body: RenewWalletHandleTokenRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostWalletRenewResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = RenewWalletHandleTokenRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/wallet/renew',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostWalletRenewResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostWalletRenewResponse
  }

  /**
   * Start a multisig signature, or add a signature to a partially completed multisig signature object.
   */
  async signMultisigProgram(
    params?: { body: SignProgramMultisigRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostMultisigProgramSignResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = SignProgramMultisigRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/multisig/signprogram',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostMultisigProgramSignResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostMultisigProgramSignResponse
  }

  /**
   * Start a multisig signature, or add a signature to a partially completed multisig signature object.
   */
  async signMultisigTransaction(
    params?: { body: SignMultisigRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostMultisigTransactionSignResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = SignMultisigRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/multisig/sign',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostMultisigTransactionSignResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostMultisigTransactionSignResponse
  }

  /**
   * Signs the passed program with a key from the wallet, determined by the account named in the request.
   */
  async signProgram(params?: { body: SignProgramRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostProgramSignResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = SignProgramRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/program/sign',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostProgramSignResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostProgramSignResponse
  }

  /**
   * Signs the passed transaction with a key from the wallet, determined by the sender encoded in the transaction.
   */
  async signTransaction(
    params?: { body: SignTransactionRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostTransactionSignResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = SignTransactionRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'POST',
      url: '/v1/transaction/sign',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1PostTransactionSignResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostTransactionSignResponse
  }

  /**
   * Returns the entire swagger spec in json.
   */
  async swaggerHandler(requestOptions?: ApiRequestOptions): Promise<string> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const payload = await this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/swagger.json',
      path: {},
      query: {},
      headers,
      body: undefined,
      mediaType: undefined,
      ...(requestOptions ?? {}),
    })

    const responseMeta = undefined
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as string
  }
}
