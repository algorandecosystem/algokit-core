import type { ModelMetadata } from '../core/model-runtime'
import type { Apiv1WalletHandle } from './apiv1-wallet-handle'
import { Apiv1WalletHandleMeta } from './apiv1-wallet-handle'

/**
 * APIV1POSTWalletInfoResponse is the response to `POST /v1/wallet/info`
 * friendly:WalletInfoResponse
 */
export type Apiv1PostWalletInfoResponse = {
  error?: boolean
  message?: string
  walletHandle?: Apiv1WalletHandle
}

export const Apiv1PostWalletInfoResponseMeta: ModelMetadata = {
  name: 'Apiv1PostWalletInfoResponse',
  kind: 'object',
  fields: [
    {
      name: 'error',
      wireKey: 'error',
      optional: true,
      nullable: false,
      type: { kind: 'scalar' },
    },
    {
      name: 'message',
      wireKey: 'message',
      optional: true,
      nullable: false,
      type: { kind: 'scalar' },
    },
    {
      name: 'walletHandle',
      wireKey: 'wallet_handle',
      optional: true,
      nullable: false,
      type: { kind: 'model', meta: () => Apiv1WalletHandleMeta },
    },
  ],
}
