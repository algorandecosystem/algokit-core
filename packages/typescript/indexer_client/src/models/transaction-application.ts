import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { BoxReference, BoxReferenceDto } from './box-reference'
import { BoxReference as BoxReferenceModel } from './box-reference'
import type { OnCompletion, OnCompletionDto } from './on-completion'
import { OnCompletion as OnCompletionModel } from './on-completion'
import type { ResourceRef, ResourceRefDto } from './resource-ref'
import { ResourceRef as ResourceRefModel } from './resource-ref'
import type { StateSchema, StateSchemaDto } from './state-schema'
import { StateSchema as StateSchemaModel } from './state-schema'

/**
 * Fields for application transactions.
 *
 * Definition:
 * data/transactions/application.go : ApplicationCallTxnFields
 */
export type TransactionApplication = {
  /**
   * \[apid\] ID of the application being configured or empty if creating.
   */
  applicationId: bigint
  onCompletion: OnCompletion

  /**
   * \[apaa\] transaction specific arguments accessed from the application's approval-program and clear-state-program.
   */
  applicationArgs?: string[]

  /**
   * \[al\] Access unifies `accounts`, `foreign-apps`, `foreign-assets`, and `box-references` under a single list. If access is non-empty, these lists must be empty. If access is empty, those lists may be non-empty.
   */
  access?: ResourceRef[]

  /**
   * \[apat\] List of accounts in addition to the sender that may be accessed from the application's approval-program and clear-state-program.
   */
  accounts?: string[]

  /**
   * \[apbx\] the boxes that can be accessed by this transaction (and others in the same group).
   */
  boxReferences?: BoxReference[]

  /**
   * \[apfa\] Lists the applications in addition to the application-id whose global states may be accessed by this application's approval-program and clear-state-program. The access is read-only.
   */
  foreignApps?: bigint[]

  /**
   * \[apas\] lists the assets whose parameters may be accessed by this application's ApprovalProgram and ClearStateProgram. The access is read-only.
   */
  foreignAssets?: bigint[]
  localStateSchema?: StateSchema
  globalStateSchema?: StateSchema

  /**
   * \[apap\] Logic executed for every application transaction, except when on-completion is set to "clear". It can read and write global state for the application, as well as account-specific local state. Approval programs may reject the transaction.
   */
  approvalProgram?: Uint8Array

  /**
   * \[apsu\] Logic executed for application transactions with on-completion set to "clear". It can read and write global state for the application, as well as account-specific local state. Clear state programs cannot reject the transaction.
   */
  clearStateProgram?: Uint8Array

  /**
   * \[epp\] specifies the additional app program len requested in pages.
   */
  extraProgramPages?: bigint

  /**
   * \[aprv\] the lowest application version for which this transaction should immediately fail. 0 indicates that no version check should be performed.
   */
  rejectVersion?: bigint
}

// JSON DTO shape for TransactionApplication with wire keys and JSON-safe primitives
export type TransactionApplicationDto = {
  'application-id': bigint
  'on-completion': OnCompletionDto
  'application-args'?: string[]
  access?: ResourceRefDto[]
  accounts?: string[]
  'box-references'?: BoxReferenceDto[]
  'foreign-apps'?: bigint[]
  'foreign-assets'?: bigint[]
  'local-state-schema'?: StateSchemaDto
  'global-state-schema'?: StateSchemaDto
  'approval-program'?: string
  'clear-state-program'?: string
  'extra-program-pages'?: bigint
  'reject-version'?: bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: TransactionApplication): TransactionApplicationDto {
  const out: any = {}
  {
    const v = (value as any)['applicationId']
    if (v === undefined) {
      // omit undefined
    } else {
      out['application-id'] = v
    }
  }
  {
    const v = (value as any)['onCompletion']
    if (v === undefined) {
      // omit undefined
    } else {
      out['on-completion'] = v === undefined ? v : OnCompletionModel.toDto(v)
    }
  }
  {
    const v = (value as any)['applicationArgs']
    if (v === undefined) {
      // omit undefined
    } else {
      out['application-args'] = v as any[]
    }
  }
  {
    const v = (value as any)['access']
    if (v === undefined) {
      // omit undefined
    } else {
      out['access'] = (v as any[]).map((item) => ResourceRefModel.toDto(item))
    }
  }
  {
    const v = (value as any)['accounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['accounts'] = v as any[]
    }
  }
  {
    const v = (value as any)['boxReferences']
    if (v === undefined) {
      // omit undefined
    } else {
      out['box-references'] = (v as any[]).map((item) => BoxReferenceModel.toDto(item))
    }
  }
  {
    const v = (value as any)['foreignApps']
    if (v === undefined) {
      // omit undefined
    } else {
      out['foreign-apps'] = v as any[]
    }
  }
  {
    const v = (value as any)['foreignAssets']
    if (v === undefined) {
      // omit undefined
    } else {
      out['foreign-assets'] = v as any[]
    }
  }
  {
    const v = (value as any)['localStateSchema']
    if (v === undefined) {
      // omit undefined
    } else {
      out['local-state-schema'] = v === undefined ? v : StateSchemaModel.toDto(v)
    }
  }
  {
    const v = (value as any)['globalStateSchema']
    if (v === undefined) {
      // omit undefined
    } else {
      out['global-state-schema'] = v === undefined ? v : StateSchemaModel.toDto(v)
    }
  }
  {
    const v = (value as any)['approvalProgram']
    if (v === undefined) {
      // omit undefined
    } else {
      out['approval-program'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['clearStateProgram']
    if (v === undefined) {
      // omit undefined
    } else {
      out['clear-state-program'] = v === undefined ? v : toBase64(v as Uint8Array)
    }
  }
  {
    const v = (value as any)['extraProgramPages']
    if (v === undefined) {
      // omit undefined
    } else {
      out['extra-program-pages'] = v
    }
  }
  {
    const v = (value as any)['rejectVersion']
    if (v === undefined) {
      // omit undefined
    } else {
      out['reject-version'] = v
    }
  }
  return out as TransactionApplicationDto
}

export function fromDto(dto: TransactionApplicationDto): TransactionApplication {
  const out: any = {}
  {
    const v = (dto as any)['application-id']
    if (v === undefined) {
      // omit undefined
    } else {
      out['applicationId'] = v as any
    }
  }
  {
    const v = (dto as any)['on-completion']
    if (v === undefined) {
      // omit undefined
    } else {
      out['onCompletion'] = v === undefined ? v : OnCompletionModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['application-args']
    if (v === undefined) {
      // omit undefined
    } else {
      out['applicationArgs'] = v as any[]
    }
  }
  {
    const v = (dto as any)['access']
    if (v === undefined) {
      // omit undefined
    } else {
      out['access'] = (v as any[]).map((item) => ResourceRefModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['accounts']
    if (v === undefined) {
      // omit undefined
    } else {
      out['accounts'] = v as any[]
    }
  }
  {
    const v = (dto as any)['box-references']
    if (v === undefined) {
      // omit undefined
    } else {
      out['boxReferences'] = (v as any[]).map((item) => BoxReferenceModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['foreign-apps']
    if (v === undefined) {
      // omit undefined
    } else {
      out['foreignApps'] = v as any[]
    }
  }
  {
    const v = (dto as any)['foreign-assets']
    if (v === undefined) {
      // omit undefined
    } else {
      out['foreignAssets'] = v as any[]
    }
  }
  {
    const v = (dto as any)['local-state-schema']
    if (v === undefined) {
      // omit undefined
    } else {
      out['localStateSchema'] = v === undefined ? v : StateSchemaModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['global-state-schema']
    if (v === undefined) {
      // omit undefined
    } else {
      out['globalStateSchema'] = v === undefined ? v : StateSchemaModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['approval-program']
    if (v === undefined) {
      // omit undefined
    } else {
      out['approvalProgram'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['clear-state-program']
    if (v === undefined) {
      // omit undefined
    } else {
      out['clearStateProgram'] = v === undefined ? v : fromBase64(v as string)
    }
  }
  {
    const v = (dto as any)['extra-program-pages']
    if (v === undefined) {
      // omit undefined
    } else {
      out['extraProgramPages'] = v as any
    }
  }
  {
    const v = (dto as any)['reject-version']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rejectVersion'] = v as any
    }
  }
  return out as TransactionApplication
}

// Msgpack codecs
export function encodeMsgpack(value: TransactionApplication): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): TransactionApplication {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: TransactionApplication): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): TransactionApplication {
  return fromDto(raw as TransactionApplicationDto)
}

// Array helpers
export function encodeMsgpackArray(values: TransactionApplication[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): TransactionApplication[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: TransactionApplication[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): TransactionApplication[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type TransactionApplicationMsgpackDto = {
  'application-id': bigint
  'on-completion': ReturnType<(typeof OnCompletionModel)['toMsgpackDto']>
  'application-args'?: string[]
  access?: ReturnType<(typeof ResourceRefModel)['toMsgpackDto']>[]
  accounts?: string[]
  'box-references'?: ReturnType<(typeof BoxReferenceModel)['toMsgpackDto']>[]
  'foreign-apps'?: bigint[]
  'foreign-assets'?: bigint[]
  'local-state-schema'?: ReturnType<(typeof StateSchemaModel)['toMsgpackDto']>
  'global-state-schema'?: ReturnType<(typeof StateSchemaModel)['toMsgpackDto']>
  'approval-program'?: Uint8Array
  'clear-state-program'?: Uint8Array
  'extra-program-pages'?: bigint
  'reject-version'?: bigint
}

function toMsgpackDto(value: TransactionApplication): TransactionApplicationMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['applicationId']
    if (v === undefined) {
    } else {
      out['application-id'] = v
    }
  }
  {
    const v = (value as any)['onCompletion']
    if (v === undefined) {
    } else {
      out['on-completion'] = OnCompletionModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['applicationArgs']
    if (v === undefined) {
    } else {
      out['application-args'] = v as any[]
    }
  }
  {
    const v = (value as any)['access']
    if (v === undefined) {
    } else {
      out['access'] = (v as any[]).map((item) => ResourceRefModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['accounts']
    if (v === undefined) {
    } else {
      out['accounts'] = v as any[]
    }
  }
  {
    const v = (value as any)['boxReferences']
    if (v === undefined) {
    } else {
      out['box-references'] = (v as any[]).map((item) => BoxReferenceModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['foreignApps']
    if (v === undefined) {
    } else {
      out['foreign-apps'] = v as any[]
    }
  }
  {
    const v = (value as any)['foreignAssets']
    if (v === undefined) {
    } else {
      out['foreign-assets'] = v as any[]
    }
  }
  {
    const v = (value as any)['localStateSchema']
    if (v === undefined) {
    } else {
      out['local-state-schema'] = StateSchemaModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['globalStateSchema']
    if (v === undefined) {
    } else {
      out['global-state-schema'] = StateSchemaModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['approvalProgram']
    if (v === undefined) {
    } else {
      out['approval-program'] = v
    }
  }
  {
    const v = (value as any)['clearStateProgram']
    if (v === undefined) {
    } else {
      out['clear-state-program'] = v
    }
  }
  {
    const v = (value as any)['extraProgramPages']
    if (v === undefined) {
    } else {
      out['extra-program-pages'] = v
    }
  }
  {
    const v = (value as any)['rejectVersion']
    if (v === undefined) {
    } else {
      out['reject-version'] = v
    }
  }
  return out as TransactionApplicationMsgpackDto
}

function fromMsgpackDto(dto: TransactionApplicationMsgpackDto): TransactionApplication {
  const out: any = {}
  {
    const v = (dto as any)['application-id']
    if (v === undefined) {
    } else {
      out['applicationId'] = v
    }
  }
  {
    const v = (dto as any)['on-completion']
    if (v === undefined) {
    } else {
      out['onCompletion'] = OnCompletionModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['application-args']
    if (v === undefined) {
    } else {
      out['applicationArgs'] = v as any[]
    }
  }
  {
    const v = (dto as any)['access']
    if (v === undefined) {
    } else {
      out['access'] = (v as any[]).map((item) => ResourceRefModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['accounts']
    if (v === undefined) {
    } else {
      out['accounts'] = v as any[]
    }
  }
  {
    const v = (dto as any)['box-references']
    if (v === undefined) {
    } else {
      out['boxReferences'] = (v as any[]).map((item) => BoxReferenceModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['foreign-apps']
    if (v === undefined) {
    } else {
      out['foreignApps'] = v as any[]
    }
  }
  {
    const v = (dto as any)['foreign-assets']
    if (v === undefined) {
    } else {
      out['foreignAssets'] = v as any[]
    }
  }
  {
    const v = (dto as any)['local-state-schema']
    if (v === undefined) {
    } else {
      out['localStateSchema'] = StateSchemaModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['global-state-schema']
    if (v === undefined) {
    } else {
      out['globalStateSchema'] = StateSchemaModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['approval-program']
    if (v === undefined) {
    } else {
      out['approvalProgram'] = v
    }
  }
  {
    const v = (dto as any)['clear-state-program']
    if (v === undefined) {
    } else {
      out['clearStateProgram'] = v
    }
  }
  {
    const v = (dto as any)['extra-program-pages']
    if (v === undefined) {
    } else {
      out['extraProgramPages'] = v
    }
  }
  {
    const v = (dto as any)['reject-version']
    if (v === undefined) {
    } else {
      out['rejectVersion'] = v
    }
  }
  return out as TransactionApplication
}

export const TransactionApplication = {
  toDto,
  fromDto,
  encodeMsgpack,
  decodeMsgpack,
  encodeJson,
  decodeJson,
  toMsgpackDto,
  fromMsgpackDto,
  encodeMsgpackArray,
  decodeMsgpackArray,
  encodeJsonArray,
  decodeJsonArray,
} as const
