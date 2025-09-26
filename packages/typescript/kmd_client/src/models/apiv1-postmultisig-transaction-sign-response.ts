import type { ModelMetadata } from '../core/model-runtime'

/**
 * APIV1POSTMultisigTransactionSignResponse is the response to `POST /v1/multisig/sign`
 * friendly:SignMultisigResponse
 */
export type Apiv1PostmultisigTransactionSignResponse = {
  error?: boolean
  message?: string
  multisig?: Uint8Array
}

export const Apiv1PostmultisigTransactionSignResponseMeta: ModelMetadata = {
  name: 'Apiv1PostmultisigTransactionSignResponse',
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
      name: 'multisig',
      wireKey: 'multisig',
      optional: true,
      nullable: false,
      type: { kind: 'scalar', isBytes: true },
    },
  ],
}
