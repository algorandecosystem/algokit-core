import type { ModelMetadata } from '../core/model-runtime'
import type { Apiv1Wallet } from './apiv1-wallet'
import { Apiv1WalletMeta } from './apiv1-wallet'

/**
 * APIV1POSTWalletRenameResponse is the response to `POST /v1/wallet/rename`
 * friendly:RenameWalletResponse
 */
export type Apiv1PostWalletRenameResponse = {
  error?: boolean
  message?: string
  wallet?: Apiv1Wallet
}

export const Apiv1PostWalletRenameResponseMeta: ModelMetadata = {
  name: 'Apiv1PostWalletRenameResponse',
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
      name: 'wallet',
      wireKey: 'wallet',
      optional: true,
      nullable: false,
      type: { kind: 'model', meta: () => Apiv1WalletMeta },
    },
  ],
}
