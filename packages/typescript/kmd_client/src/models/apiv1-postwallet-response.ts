import type { ModelMetadata } from '../core/model-runtime'
import type { Apiv1Wallet } from './apiv1-wallet'
import { Apiv1WalletMeta } from './apiv1-wallet'

/**
 * APIV1POSTWalletResponse is the response to `POST /v1/wallet`
 * friendly:CreateWalletResponse
 */
export type Apiv1PostwalletResponse = {
  error?: boolean
  message?: string
  wallet?: Apiv1Wallet
}

export const Apiv1PostwalletResponseMeta: ModelMetadata = {
  name: 'Apiv1PostwalletResponse',
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
