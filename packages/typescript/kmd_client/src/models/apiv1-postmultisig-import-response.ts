import type { ModelMetadata } from '../core/model-runtime'

/**
 * APIV1POSTMultisigImportResponse is the response to `POST /v1/multisig/import`
 * friendly:ImportMultisigResponse
 */
export type Apiv1PostmultisigImportResponse = {
  address?: string
  error?: boolean
  message?: string
}

export const Apiv1PostmultisigImportResponseMeta: ModelMetadata = {
  name: 'Apiv1PostmultisigImportResponse',
  kind: 'object',
  fields: [
    {
      name: 'address',
      wireKey: 'address',
      optional: true,
      nullable: false,
      type: { kind: 'scalar' },
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
