import type { ModelMetadata } from '../core/model-runtime'
import type { Apiv1WalletHandle } from './apiv1-wallet-handle'
import { Apiv1WalletHandleMeta } from './apiv1-wallet-handle'

/**
 * APIV1POSTWalletRenewResponse is the response to `POST /v1/wallet/renew`
 * friendly:RenewWalletHandleTokenResponse
 */
export type Apiv1PostwalletRenewResponse = {
  error?: boolean
  message?: string
  walletHandle?: Apiv1WalletHandle
}

export const Apiv1PostwalletRenewResponseMeta: ModelMetadata = {
  name: 'Apiv1PostwalletRenewResponse',
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
