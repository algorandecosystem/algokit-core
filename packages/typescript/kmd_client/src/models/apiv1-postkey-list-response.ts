import type { ModelMetadata } from '../core/model-runtime'

/**
 * APIV1POSTKeyListResponse is the response to `POST /v1/key/list`
 * friendly:ListKeysResponse
 */
export type Apiv1PostkeyListResponse = {
  addresses?: string[]
  error?: boolean
  message?: string
}

export const Apiv1PostkeyListResponseMeta: ModelMetadata = {
  name: 'Apiv1PostkeyListResponse',
  kind: 'object',
  fields: [
    {
      name: 'addresses',
      wireKey: 'addresses',
      optional: true,
      nullable: false,
      type: { kind: 'array', item: { kind: 'scalar' } },
    },
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
