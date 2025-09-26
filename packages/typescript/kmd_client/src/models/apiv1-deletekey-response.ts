import type { ModelMetadata } from '../core/model-runtime'

/**
 * APIV1DELETEKeyResponse is the response to `DELETE /v1/key`
 * friendly:DeleteKeyResponse
 */
export type Apiv1DeletekeyResponse = {
  error?: boolean
  message?: string
}

export const Apiv1DeletekeyResponseMeta: ModelMetadata = {
  name: 'Apiv1DeletekeyResponse',
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
