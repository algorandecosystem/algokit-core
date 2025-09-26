import type { ModelMetadata } from '../core/model-runtime'

/**
 * APIV1POSTWalletInitResponse is the response to `POST /v1/wallet/init`
 * friendly:InitWalletHandleTokenResponse
 */
export type Apiv1PostwalletInitResponse = {
  error?: boolean
  message?: string
  walletHandleToken?: string
}

export const Apiv1PostwalletInitResponseMeta: ModelMetadata = {
  name: 'Apiv1PostwalletInitResponse',
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
      name: 'walletHandleToken',
      wireKey: 'wallet_handle_token',
      optional: true,
      nullable: false,
      type: { kind: 'scalar' },
    },
  ],
}
