import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { ApplicationStateSchema, ApplicationStateSchemaDto } from './application-state-schema'
import { ApplicationStateSchema as ApplicationStateSchemaModel } from './application-state-schema'
import type { TealKeyValueStore, TealKeyValueStoreDto } from './teal-key-value-store'
import { TealKeyValueStore as TealKeyValueStoreModel } from './teal-key-value-store'

/**
 * Stores the global information associated with an application.
 */
export type ApplicationParams = {
  /**
   * The address that created this application. This is the address where the parameters and global state for this application can be found.
   */
  creator?: string

  /**
   * approval program.
   */
  approvalProgram?: Uint8Array

  /**
   * clear state program.
   */
  clearStateProgram?: Uint8Array

  /**
   * the number of extra program pages available to this app.
   */
  extraProgramPages?: bigint
  localStateSchema?: ApplicationStateSchema
  globalStateSchema?: ApplicationStateSchema
  globalState?: TealKeyValueStore

  /**
   * the number of updates to the application programs
   */
  version?: bigint
}

// JSON DTO shape for ApplicationParams with wire keys and JSON-safe primitives
export type ApplicationParamsDto = {
  creator?: string
  'approval-program'?: string
  'clear-state-program'?: string
  'extra-program-pages'?: bigint
  'local-state-schema'?: ApplicationStateSchemaDto
  'global-state-schema'?: ApplicationStateSchemaDto
  'global-state'?: TealKeyValueStoreDto
  version?: bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: ApplicationParams): ApplicationParamsDto {
  const out: any = {}
  {
    const v = (value as any)['creator']
    if (v === undefined) {
      // omit undefined
    } else {
      out['creator'] = v
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
    const v = (value as any)['localStateSchema']
    if (v === undefined) {
      // omit undefined
    } else {
      out['local-state-schema'] = v === undefined ? v : ApplicationStateSchemaModel.toDto(v)
    }
  }
  {
    const v = (value as any)['globalStateSchema']
    if (v === undefined) {
      // omit undefined
    } else {
      out['global-state-schema'] = v === undefined ? v : ApplicationStateSchemaModel.toDto(v)
    }
  }
  {
    const v = (value as any)['globalState']
    if (v === undefined) {
      // omit undefined
    } else {
      out['global-state'] = v === undefined ? v : TealKeyValueStoreModel.toDto(v)
    }
  }
  {
    const v = (value as any)['version']
    if (v === undefined) {
      // omit undefined
    } else {
      out['version'] = v
    }
  }
  return out as ApplicationParamsDto
}

export function fromDto(dto: ApplicationParamsDto): ApplicationParams {
  const out: any = {}
  {
    const v = (dto as any)['creator']
    if (v === undefined) {
      // omit undefined
    } else {
      out['creator'] = v as any
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
    const v = (dto as any)['local-state-schema']
    if (v === undefined) {
      // omit undefined
    } else {
      out['localStateSchema'] = v === undefined ? v : ApplicationStateSchemaModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['global-state-schema']
    if (v === undefined) {
      // omit undefined
    } else {
      out['globalStateSchema'] = v === undefined ? v : ApplicationStateSchemaModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['global-state']
    if (v === undefined) {
      // omit undefined
    } else {
      out['globalState'] = v === undefined ? v : TealKeyValueStoreModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['version']
    if (v === undefined) {
      // omit undefined
    } else {
      out['version'] = v as any
    }
  }
  return out as ApplicationParams
}

// Msgpack codecs
export function encodeMsgpack(value: ApplicationParams): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): ApplicationParams {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: ApplicationParams): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): ApplicationParams {
  return fromDto(raw as ApplicationParamsDto)
}

// Array helpers
export function encodeMsgpackArray(values: ApplicationParams[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): ApplicationParams[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: ApplicationParams[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): ApplicationParams[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type ApplicationParamsMsgpackDto = {
  creator?: string
  'approval-program'?: Uint8Array
  'clear-state-program'?: Uint8Array
  'extra-program-pages'?: bigint
  'local-state-schema'?: ReturnType<(typeof ApplicationStateSchemaModel)['toMsgpackDto']>
  'global-state-schema'?: ReturnType<(typeof ApplicationStateSchemaModel)['toMsgpackDto']>
  'global-state'?: ReturnType<(typeof TealKeyValueStoreModel)['toMsgpackDto']>
  version?: bigint
}

function toMsgpackDto(value: ApplicationParams): ApplicationParamsMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['creator']
    if (v === undefined) {
    } else {
      out['creator'] = v
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
    const v = (value as any)['localStateSchema']
    if (v === undefined) {
    } else {
      out['local-state-schema'] = ApplicationStateSchemaModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['globalStateSchema']
    if (v === undefined) {
    } else {
      out['global-state-schema'] = ApplicationStateSchemaModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['globalState']
    if (v === undefined) {
    } else {
      out['global-state'] = TealKeyValueStoreModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['version']
    if (v === undefined) {
    } else {
      out['version'] = v
    }
  }
  return out as ApplicationParamsMsgpackDto
}

function fromMsgpackDto(dto: ApplicationParamsMsgpackDto): ApplicationParams {
  const out: any = {}
  {
    const v = (dto as any)['creator']
    if (v === undefined) {
    } else {
      out['creator'] = v
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
    const v = (dto as any)['local-state-schema']
    if (v === undefined) {
    } else {
      out['localStateSchema'] = ApplicationStateSchemaModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['global-state-schema']
    if (v === undefined) {
    } else {
      out['globalStateSchema'] = ApplicationStateSchemaModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['global-state']
    if (v === undefined) {
    } else {
      out['globalState'] = TealKeyValueStoreModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['version']
    if (v === undefined) {
    } else {
      out['version'] = v
    }
  }
  return out as ApplicationParams
}

export const ApplicationParams = {
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
