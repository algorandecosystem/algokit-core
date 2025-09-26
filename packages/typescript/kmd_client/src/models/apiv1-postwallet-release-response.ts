import type { ModelMetadata } from '../core/model-runtime'

/**
 * APIV1POSTWalletReleaseResponse is the response to `POST /v1/wallet/release`
 * friendly:ReleaseWalletHandleTokenResponse
 */
export type Apiv1PostwalletReleaseResponse = {
  error?: boolean
  message?: string
}

export const Apiv1PostwalletReleaseResponseMeta: ModelMetadata = {
  name: 'Apiv1PostwalletReleaseResponse',
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
