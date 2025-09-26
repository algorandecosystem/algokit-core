import type { ModelMetadata } from '../core/model-runtime'

/**
 * APIV1POSTTransactionSignResponse is the response to `POST /v1/transaction/sign`
 * friendly:SignTransactionResponse
 */
export type Apiv1PosttransactionSignResponse = {
  error?: boolean
  message?: string
  signedTransaction?: Uint8Array
}

export const Apiv1PosttransactionSignResponseMeta: ModelMetadata = {
  name: 'Apiv1PosttransactionSignResponse',
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
      name: 'signedTransaction',
      wireKey: 'signed_transaction',
      optional: true,
      nullable: false,
      type: { kind: 'scalar', isBytes: true },
    },
  ],
}
