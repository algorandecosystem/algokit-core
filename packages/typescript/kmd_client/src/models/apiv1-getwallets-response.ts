import type { ModelMetadata } from '../core/model-runtime'
import type { Apiv1Wallet } from './apiv1-wallet'
import { Apiv1WalletMeta } from './apiv1-wallet'

/**
 * APIV1GETWalletsResponse is the response to `GET /v1/wallets`
 * friendly:ListWalletsResponse
 */
export type Apiv1GetwalletsResponse = {
  error?: boolean
  message?: string
  wallets?: Apiv1Wallet[]
}

export const Apiv1GetwalletsResponseMeta: ModelMetadata = {
  name: 'Apiv1GetwalletsResponse',
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
      name: 'wallets',
      wireKey: 'wallets',
      optional: true,
      nullable: false,
      type: { kind: 'array', item: { kind: 'model', meta: () => Apiv1WalletMeta } },
    },
  ],
}
