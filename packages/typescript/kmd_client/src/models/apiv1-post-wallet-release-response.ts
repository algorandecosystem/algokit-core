import type { ModelMetadata } from '../core/model-runtime'

/**
 * APIV1POSTWalletReleaseResponse is the response to `POST /v1/wallet/release`
 * friendly:ReleaseWalletHandleTokenResponse
 */
export type Apiv1PostWalletReleaseResponse = {
  error?: boolean
  message?: string
}

export const Apiv1PostWalletReleaseResponseMeta: ModelMetadata = {
  name: 'Apiv1PostWalletReleaseResponse',
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
  ],
}
