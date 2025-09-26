import type { ModelMetadata } from '../core/model-runtime'

/**
 * APIV1POSTMultisigListResponse is the response to `POST /v1/multisig/list`
 * friendly:ListMultisigResponse
 */
export type Apiv1PostmultisigListResponse = {
  addresses?: string[]
  error?: boolean
  message?: string
}

export const Apiv1PostmultisigListResponseMeta: ModelMetadata = {
  name: 'Apiv1PostmultisigListResponse',
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
