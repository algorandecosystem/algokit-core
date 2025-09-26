import type { ModelMetadata } from '../core/model-runtime'

/**
 * APIV1POSTKeyImportResponse is the response to `POST /v1/key/import`
 * friendly:ImportKeyResponse
 */
export type Apiv1PostkeyImportResponse = {
  address?: string
  error?: boolean
  message?: string
}

export const Apiv1PostkeyImportResponseMeta: ModelMetadata = {
  name: 'Apiv1PostkeyImportResponse',
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
