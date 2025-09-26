import type { BaseHttpRequest, ApiRequestOptions } from '../core/base-http-request'
import { AlgorandSerializer } from '../core/model-runtime'
import type {
  Apiv1DeletekeyResponse,
  Apiv1DeletemultisigResponse,
  Apiv1GetwalletsResponse,
  Apiv1PostkeyExportResponse,
  Apiv1PostkeyImportResponse,
  Apiv1PostkeyListResponse,
  Apiv1PostkeyResponse,
  Apiv1PostmasterKeyExportResponse,
  Apiv1PostmultisigExportResponse,
  Apiv1PostmultisigImportResponse,
  Apiv1PostmultisigListResponse,
  Apiv1PostmultisigProgramSignResponse,
  Apiv1PostmultisigTransactionSignResponse,
  Apiv1PostprogramSignResponse,
  Apiv1PosttransactionSignResponse,
  Apiv1PostwalletInfoResponse,
  Apiv1PostwalletInitResponse,
  Apiv1PostwalletReleaseResponse,
  Apiv1PostwalletRenameResponse,
  Apiv1PostwalletRenewResponse,
  Apiv1PostwalletResponse,
  CreateWalletRequest,
  DeleteKeyRequest,
  DeleteMultisigRequest,
  ExportKeyRequest,
  ExportMasterKeyRequest,
  ExportMultisigRequest,
  GenerateKeyRequest,
  ImportKeyRequest,
  ImportMultisigRequest,
  InitWalletHandleTokenRequest,
  ListKeysRequest,
  ListMultisigRequest,
  ListWalletsRequest,
  ReleaseWalletHandleTokenRequest,
  RenameWalletRequest,
  RenewWalletHandleTokenRequest,
  SignMultisigRequest,
  SignProgramMultisigRequest,
  SignProgramRequest,
  SignTransactionRequest,
  VersionsRequest,
  VersionsResponse,
  WalletInfoRequest,
} from '../models/index'
import {
  Apiv1DeletekeyResponseMeta,
  Apiv1DeletemultisigResponseMeta,
  Apiv1GetwalletsResponseMeta,
  Apiv1PostkeyExportResponseMeta,
  Apiv1PostkeyImportResponseMeta,
  Apiv1PostkeyListResponseMeta,
  Apiv1PostkeyResponseMeta,
  Apiv1PostmasterKeyExportResponseMeta,
  Apiv1PostmultisigExportResponseMeta,
  Apiv1PostmultisigImportResponseMeta,
  Apiv1PostmultisigListResponseMeta,
  Apiv1PostmultisigProgramSignResponseMeta,
  Apiv1PostmultisigTransactionSignResponseMeta,
  Apiv1PostprogramSignResponseMeta,
  Apiv1PosttransactionSignResponseMeta,
  Apiv1PostwalletInfoResponseMeta,
  Apiv1PostwalletInitResponseMeta,
  Apiv1PostwalletReleaseResponseMeta,
  Apiv1PostwalletRenameResponseMeta,
  Apiv1PostwalletRenewResponseMeta,
  Apiv1PostwalletResponseMeta,
  CreateWalletRequestMeta,
  DeleteKeyRequestMeta,
  DeleteMultisigRequestMeta,
  ExportKeyRequestMeta,
  ExportMasterKeyRequestMeta,
  ExportMultisigRequestMeta,
  GenerateKeyRequestMeta,
  ImportKeyRequestMeta,
  ImportMultisigRequestMeta,
  InitWalletHandleTokenRequestMeta,
  ListKeysRequestMeta,
  ListMultisigRequestMeta,
  ListWalletsRequestMeta,
  ReleaseWalletHandleTokenRequestMeta,
  RenameWalletRequestMeta,
  RenewWalletHandleTokenRequestMeta,
  SignMultisigRequestMeta,
  SignProgramMultisigRequestMeta,
  SignProgramRequestMeta,
  SignTransactionRequestMeta,
  VersionsRequestMeta,
  VersionsResponseMeta,
  WalletInfoRequestMeta,
} from '../models/index'

export class KmdApi {
  constructor(public readonly httpRequest: BaseHttpRequest) {}

  /**
   * Create a new wallet (collection of keys) with the given parameters.
   */
  async createWallet(params?: { body: CreateWalletRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostwalletResponse> {
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

    const responseMeta = Apiv1PostwalletResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostwalletResponse
  }

  /**
   * Deletes the key with the passed public key from the wallet.
   */
  async deleteKey(params?: { body: DeleteKeyRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1DeletekeyResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = DeleteKeyRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'DELETE',
      url: '/v1/key',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1DeletekeyResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1DeletekeyResponse
  }

  /**
   * Deletes multisig preimage information for the passed address from the wallet.
   */
  async deleteMultisig(params?: { body: DeleteMultisigRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1DeletemultisigResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = DeleteMultisigRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'DELETE',
      url: '/v1/multisig',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1DeletemultisigResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1DeletemultisigResponse
  }

  /**
   * Export the secret key associated with the passed public key.
   */
  async exportKey(params?: { body: ExportKeyRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostkeyExportResponse> {
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

    const responseMeta = Apiv1PostkeyExportResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostkeyExportResponse
  }

  /**
   * Export the master derivation key from the wallet. This key is a master "backup" key for the underlying wallet. With it, you can regenerate all of the wallets that have been generated with this wallet's `POST /v1/key` endpoint. This key will not allow you to recover keys imported from other wallets, however.
   */
  async exportMasterKey(
    params?: { body: ExportMasterKeyRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostmasterKeyExportResponse> {
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

    const responseMeta = Apiv1PostmasterKeyExportResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostmasterKeyExportResponse
  }

  /**
   * Given a multisig address whose preimage this wallet stores, returns the information used to generate the address, including public keys, threshold, and multisig version.
   */
  async exportMultisig(
    params?: { body: ExportMultisigRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostmultisigExportResponse> {
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

    const responseMeta = Apiv1PostmultisigExportResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostmultisigExportResponse
  }

  /**
   * Generates the next key in the deterministic key sequence (as determined by the master derivation key) and adds it to the wallet, returning the public key.
   */
  async generateKey(params?: { body: GenerateKeyRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostkeyResponse> {
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

    const responseMeta = Apiv1PostkeyResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostkeyResponse
  }

  async getVersion(params?: { body?: VersionsRequest }, requestOptions?: ApiRequestOptions): Promise<VersionsResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = VersionsRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/versions',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
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
  async getWalletInfo(params?: { body: WalletInfoRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostwalletInfoResponse> {
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

    const responseMeta = Apiv1PostwalletInfoResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostwalletInfoResponse
  }

  /**
   * Import an externally generated key into the wallet. Note that if you wish to back up the imported key, you must do so by backing up the entire wallet database, because imported keys were not derived from the wallet's master derivation key.
   */
  async importKey(params?: { body: ImportKeyRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostkeyImportResponse> {
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

    const responseMeta = Apiv1PostkeyImportResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostkeyImportResponse
  }

  /**
   * Generates a multisig account from the passed public keys array and multisig metadata, and stores all of this in the wallet.
   */
  async importMultisig(
    params?: { body: ImportMultisigRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostmultisigImportResponse> {
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

    const responseMeta = Apiv1PostmultisigImportResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostmultisigImportResponse
  }

  /**
   * Unlock the wallet and return a wallet handle token that can be used for subsequent operations. These tokens expire periodically and must be renewed. You can `POST` the token to `/v1/wallet/info` to see how much time remains until expiration, and renew it with `/v1/wallet/renew`. When you're done, you can invalidate the token with `/v1/wallet/release`.
   */
  async initWalletHandleToken(
    params?: { body: InitWalletHandleTokenRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostwalletInitResponse> {
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

    const responseMeta = Apiv1PostwalletInitResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostwalletInitResponse
  }

  /**
   * Lists all of the public keys in this wallet. All of them have a stored private key.
   */
  async listKeysInWallet(params?: { body: ListKeysRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostkeyListResponse> {
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

    const responseMeta = Apiv1PostkeyListResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostkeyListResponse
  }

  /**
   * Lists all of the multisig accounts whose preimages this wallet stores
   */
  async listMultisg(params?: { body: ListMultisigRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostmultisigListResponse> {
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

    const responseMeta = Apiv1PostmultisigListResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostmultisigListResponse
  }

  /**
   * Lists all of the wallets that kmd is aware of.
   */
  async listWallets(params?: { body?: ListWalletsRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1GetwalletsResponse> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const bodyMeta = ListWalletsRequestMeta
    const mediaType = bodyMeta ? (responseFormat === 'json' ? 'application/json' : 'application/msgpack') : undefined
    if (mediaType) headers['Content-Type'] = mediaType
    const serializedBody =
      bodyMeta && params?.body !== undefined ? AlgorandSerializer.encode(params.body, bodyMeta, responseFormat) : params?.body

    const payload = await this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/v1/wallets',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = Apiv1GetwalletsResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1GetwalletsResponse
  }

  /**
   * Invalidate the passed wallet handle token, making it invalid for use in subsequent requests.
   */
  async releaseWalletHandleToken(
    params?: { body: ReleaseWalletHandleTokenRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostwalletReleaseResponse> {
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

    const responseMeta = Apiv1PostwalletReleaseResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostwalletReleaseResponse
  }

  /**
   * Rename the underlying wallet to something else
   */
  async renameWallet(params?: { body: RenameWalletRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostwalletRenameResponse> {
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

    const responseMeta = Apiv1PostwalletRenameResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostwalletRenameResponse
  }

  /**
   * Renew a wallet handle token, increasing its expiration duration to its initial value
   */
  async renewWalletHandleToken(
    params?: { body: RenewWalletHandleTokenRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostwalletRenewResponse> {
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

    const responseMeta = Apiv1PostwalletRenewResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostwalletRenewResponse
  }

  /**
   * Start a multisig signature, or add a signature to a partially completed multisig signature object.
   */
  async signMultisigProgram(
    params?: { body: SignProgramMultisigRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostmultisigProgramSignResponse> {
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

    const responseMeta = Apiv1PostmultisigProgramSignResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostmultisigProgramSignResponse
  }

  /**
   * Start a multisig signature, or add a signature to a partially completed multisig signature object.
   */
  async signMultisigTransaction(
    params?: { body: SignMultisigRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PostmultisigTransactionSignResponse> {
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

    const responseMeta = Apiv1PostmultisigTransactionSignResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostmultisigTransactionSignResponse
  }

  /**
   * Signs the passed program with a key from the wallet, determined by the account named in the request.
   */
  async signProgram(params?: { body: SignProgramRequest }, requestOptions?: ApiRequestOptions): Promise<Apiv1PostprogramSignResponse> {
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

    const responseMeta = Apiv1PostprogramSignResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PostprogramSignResponse
  }

  /**
   * Signs the passed transaction with a key from the wallet, determined by the sender encoded in the transaction.
   */
  async signTransaction(
    params?: { body: SignTransactionRequest },
    requestOptions?: ApiRequestOptions,
  ): Promise<Apiv1PosttransactionSignResponse> {
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

    const responseMeta = Apiv1PosttransactionSignResponseMeta
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as Apiv1PosttransactionSignResponse
  }

  /**
   * Returns the entire swagger spec in json.
   */
  async swaggerHandler(requestOptions?: ApiRequestOptions): Promise<string> {
    const headers: Record<string, string> = {}
    const responseFormat: 'json' | 'msgpack' = 'json'
    headers['Accept'] = responseFormat === 'json' ? 'application/json' : 'application/msgpack'

    const serializedBody = undefined
    const mediaType = undefined

    const payload = await this.httpRequest.request<unknown>({
      method: 'GET',
      url: '/swagger.json',
      path: {},
      query: {},
      headers,
      body: serializedBody,
      mediaType: mediaType,
      ...(requestOptions ?? {}),
    })

    const responseMeta = undefined
    if (responseMeta) {
      return AlgorandSerializer.decode(payload, responseMeta, responseFormat)
    }
    return payload as string
  }
}
