import type { ModelMetadata } from '../core/model-runtime'
import type { Apiv1Wallet } from './apiv1-wallet'
import { Apiv1WalletMeta } from './apiv1-wallet'

/**
 * APIV1WalletHandle includes the wallet the handle corresponds to
 * and the number of number of seconds to expiration
 */
export type Apiv1WalletHandle = {
  expiresSeconds?: bigint
  wallet?: Apiv1Wallet
}

export const Apiv1WalletHandleMeta: ModelMetadata = {
  name: 'Apiv1WalletHandle',
  kind: 'object',
  fields: [
    {
      name: 'expiresSeconds',
      wireKey: 'expires_seconds',
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
