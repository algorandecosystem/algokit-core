import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { ApplicationLocalState, ApplicationLocalStateDto } from './application-local-state'
import { ApplicationLocalState as ApplicationLocalStateModel } from './application-local-state'
import type { ApplicationParams, ApplicationParamsDto } from './application-params'
import { ApplicationParams as ApplicationParamsModel } from './application-params'

export type AccountApplicationInformation = {
  /**
   * The round for which this information is relevant.
   */
  round: bigint
  appLocalState?: ApplicationLocalState
  createdApp?: ApplicationParams
}

// JSON DTO shape for AccountApplicationInformation with wire keys and JSON-safe primitives
export type AccountApplicationInformationDto = {
  round: string
  'app-local-state'?: ApplicationLocalStateDto
  'created-app'?: ApplicationParamsDto
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: AccountApplicationInformation): AccountApplicationInformationDto {
  const out: any = {}
  {
    const v = (value as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['appLocalState']
    if (v === undefined) {
      // omit undefined
    } else {
      out['app-local-state'] = v === undefined ? v : ApplicationLocalStateModel.toDto(v)
    }
  }
  {
    const v = (value as any)['createdApp']
    if (v === undefined) {
      // omit undefined
    } else {
      out['created-app'] = v === undefined ? v : ApplicationParamsModel.toDto(v)
    }
  }
  return out as AccountApplicationInformationDto
}

export function fromDto(dto: AccountApplicationInformationDto): AccountApplicationInformation {
  const out: any = {}
  {
    const v = (dto as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['app-local-state']
    if (v === undefined) {
      // omit undefined
    } else {
      out['appLocalState'] = v === undefined ? v : ApplicationLocalStateModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['created-app']
    if (v === undefined) {
      // omit undefined
    } else {
      out['createdApp'] = v === undefined ? v : ApplicationParamsModel.fromDto(v)
    }
  }
  return out as AccountApplicationInformation
}

// Msgpack codecs
export function encodeMsgpack(value: AccountApplicationInformation): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): AccountApplicationInformation {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: AccountApplicationInformation): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): AccountApplicationInformation {
  return fromDto(raw as AccountApplicationInformationDto)
}

// Array helpers
export function encodeMsgpackArray(values: AccountApplicationInformation[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): AccountApplicationInformation[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: AccountApplicationInformation[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): AccountApplicationInformation[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type AccountApplicationInformationMsgpackDto = {
  round: bigint
  'app-local-state'?: ReturnType<(typeof ApplicationLocalStateModel)['toMsgpackDto']>
  'created-app'?: ReturnType<(typeof ApplicationParamsModel)['toMsgpackDto']>
}

function toMsgpackDto(value: AccountApplicationInformation): AccountApplicationInformationMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (value as any)['appLocalState']
    if (v === undefined) {
    } else {
      out['app-local-state'] = ApplicationLocalStateModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['createdApp']
    if (v === undefined) {
    } else {
      out['created-app'] = ApplicationParamsModel.toMsgpackDto(v)
    }
  }
  return out as AccountApplicationInformationMsgpackDto
}

function fromMsgpackDto(dto: AccountApplicationInformationMsgpackDto): AccountApplicationInformation {
  const out: any = {}
  {
    const v = (dto as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (dto as any)['app-local-state']
    if (v === undefined) {
    } else {
      out['appLocalState'] = ApplicationLocalStateModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['created-app']
    if (v === undefined) {
    } else {
      out['createdApp'] = ApplicationParamsModel.fromMsgpackDto(v)
    }
  }
  return out as AccountApplicationInformation
}

export const AccountApplicationInformation = {
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
