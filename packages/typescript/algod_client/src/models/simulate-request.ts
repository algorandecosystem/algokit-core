import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'
import type { SimulateRequestTransactionGroup, SimulateRequestTransactionGroupDto } from './simulate-request-transaction-group'
import { SimulateRequestTransactionGroup as SimulateRequestTransactionGroupModel } from './simulate-request-transaction-group'
import type { SimulateTraceConfig, SimulateTraceConfigDto } from './simulate-trace-config'
import { SimulateTraceConfig as SimulateTraceConfigModel } from './simulate-trace-config'

/**
 * Request type for simulation endpoint.
 */
export type SimulateRequest = {
  /**
   * The transaction groups to simulate.
   */
  txnGroups: SimulateRequestTransactionGroup[]

  /**
   * If provided, specifies the round preceding the simulation. State changes through this round will be used to run this simulation. Usually only the 4 most recent rounds will be available (controlled by the node config value MaxAcctLookback). If not specified, defaults to the latest available round.
   */
  round?: bigint

  /**
   * Allows transactions without signatures to be simulated as if they had correct signatures.
   */
  allowEmptySignatures?: boolean

  /**
   * Lifts limits on log opcode usage during simulation.
   */
  allowMoreLogging?: boolean

  /**
   * Allows access to unnamed resources during simulation.
   */
  allowUnnamedResources?: boolean

  /**
   * Applies extra opcode budget during simulation for each transaction group.
   */
  extraOpcodeBudget?: bigint
  execTraceConfig?: SimulateTraceConfig

  /**
   * If true, signers for transactions that are missing signatures will be fixed during evaluation.
   */
  fixSigners?: boolean
}

// JSON DTO shape for SimulateRequest with wire keys and JSON-safe primitives
export type SimulateRequestDto = {
  'txn-groups': SimulateRequestTransactionGroupDto[]
  round?: string
  'allow-empty-signatures'?: boolean
  'allow-more-logging'?: boolean
  'allow-unnamed-resources'?: boolean
  'extra-opcode-budget'?: bigint
  'exec-trace-config'?: SimulateTraceConfigDto
  'fix-signers'?: boolean
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: SimulateRequest): SimulateRequestDto {
  const out: any = {}
  {
    const v = (value as any)['txnGroups']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txn-groups'] = (v as any[]).map((item) => SimulateRequestTransactionGroupModel.toDto(item))
    }
  }
  {
    const v = (value as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['allowEmptySignatures']
    if (v === undefined) {
      // omit undefined
    } else {
      out['allow-empty-signatures'] = v
    }
  }
  {
    const v = (value as any)['allowMoreLogging']
    if (v === undefined) {
      // omit undefined
    } else {
      out['allow-more-logging'] = v
    }
  }
  {
    const v = (value as any)['allowUnnamedResources']
    if (v === undefined) {
      // omit undefined
    } else {
      out['allow-unnamed-resources'] = v
    }
  }
  {
    const v = (value as any)['extraOpcodeBudget']
    if (v === undefined) {
      // omit undefined
    } else {
      out['extra-opcode-budget'] = v === undefined ? v : typeof v === 'bigint' ? v.toString() : String(v)
    }
  }
  {
    const v = (value as any)['execTraceConfig']
    if (v === undefined) {
      // omit undefined
    } else {
      out['exec-trace-config'] = v === undefined ? v : SimulateTraceConfigModel.toDto(v)
    }
  }
  {
    const v = (value as any)['fixSigners']
    if (v === undefined) {
      // omit undefined
    } else {
      out['fix-signers'] = v
    }
  }
  return out as SimulateRequestDto
}

export function fromDto(dto: SimulateRequestDto): SimulateRequest {
  const out: any = {}
  {
    const v = (dto as any)['txn-groups']
    if (v === undefined) {
      // omit undefined
    } else {
      out['txnGroups'] = (v as any[]).map((item) => SimulateRequestTransactionGroupModel.fromDto(item))
    }
  }
  {
    const v = (dto as any)['round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['round'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['allow-empty-signatures']
    if (v === undefined) {
      // omit undefined
    } else {
      out['allowEmptySignatures'] = v as any
    }
  }
  {
    const v = (dto as any)['allow-more-logging']
    if (v === undefined) {
      // omit undefined
    } else {
      out['allowMoreLogging'] = v as any
    }
  }
  {
    const v = (dto as any)['allow-unnamed-resources']
    if (v === undefined) {
      // omit undefined
    } else {
      out['allowUnnamedResources'] = v as any
    }
  }
  {
    const v = (dto as any)['extra-opcode-budget']
    if (v === undefined) {
      // omit undefined
    } else {
      out['extraOpcodeBudget'] = v === undefined ? v : typeof v === 'bigint' ? v : BigInt(v as any)
    }
  }
  {
    const v = (dto as any)['exec-trace-config']
    if (v === undefined) {
      // omit undefined
    } else {
      out['execTraceConfig'] = v === undefined ? v : SimulateTraceConfigModel.fromDto(v)
    }
  }
  {
    const v = (dto as any)['fix-signers']
    if (v === undefined) {
      // omit undefined
    } else {
      out['fixSigners'] = v as any
    }
  }
  return out as SimulateRequest
}

// Msgpack codecs
export function encodeMsgpack(value: SimulateRequest): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): SimulateRequest {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: SimulateRequest): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): SimulateRequest {
  return fromDto(raw as SimulateRequestDto)
}

// Array helpers
export function encodeMsgpackArray(values: SimulateRequest[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): SimulateRequest[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: SimulateRequest[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): SimulateRequest[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type SimulateRequestMsgpackDto = {
  'txn-groups': ReturnType<(typeof SimulateRequestTransactionGroupModel)['toMsgpackDto']>[]
  round?: bigint
  'allow-empty-signatures'?: boolean
  'allow-more-logging'?: boolean
  'allow-unnamed-resources'?: boolean
  'extra-opcode-budget'?: bigint
  'exec-trace-config'?: ReturnType<(typeof SimulateTraceConfigModel)['toMsgpackDto']>
  'fix-signers'?: boolean
}

function toMsgpackDto(value: SimulateRequest): SimulateRequestMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['txnGroups']
    if (v === undefined) {
    } else {
      out['txn-groups'] = (v as any[]).map((item) => SimulateRequestTransactionGroupModel.toMsgpackDto(item))
    }
  }
  {
    const v = (value as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (value as any)['allowEmptySignatures']
    if (v === undefined) {
    } else {
      out['allow-empty-signatures'] = v
    }
  }
  {
    const v = (value as any)['allowMoreLogging']
    if (v === undefined) {
    } else {
      out['allow-more-logging'] = v
    }
  }
  {
    const v = (value as any)['allowUnnamedResources']
    if (v === undefined) {
    } else {
      out['allow-unnamed-resources'] = v
    }
  }
  {
    const v = (value as any)['extraOpcodeBudget']
    if (v === undefined) {
    } else {
      out['extra-opcode-budget'] = v
    }
  }
  {
    const v = (value as any)['execTraceConfig']
    if (v === undefined) {
    } else {
      out['exec-trace-config'] = SimulateTraceConfigModel.toMsgpackDto(v)
    }
  }
  {
    const v = (value as any)['fixSigners']
    if (v === undefined) {
    } else {
      out['fix-signers'] = v
    }
  }
  return out as SimulateRequestMsgpackDto
}

function fromMsgpackDto(dto: SimulateRequestMsgpackDto): SimulateRequest {
  const out: any = {}
  {
    const v = (dto as any)['txn-groups']
    if (v === undefined) {
    } else {
      out['txnGroups'] = (v as any[]).map((item) => SimulateRequestTransactionGroupModel.fromMsgpackDto(item))
    }
  }
  {
    const v = (dto as any)['round']
    if (v === undefined) {
    } else {
      out['round'] = v
    }
  }
  {
    const v = (dto as any)['allow-empty-signatures']
    if (v === undefined) {
    } else {
      out['allowEmptySignatures'] = v
    }
  }
  {
    const v = (dto as any)['allow-more-logging']
    if (v === undefined) {
    } else {
      out['allowMoreLogging'] = v
    }
  }
  {
    const v = (dto as any)['allow-unnamed-resources']
    if (v === undefined) {
    } else {
      out['allowUnnamedResources'] = v
    }
  }
  {
    const v = (dto as any)['extra-opcode-budget']
    if (v === undefined) {
    } else {
      out['extraOpcodeBudget'] = v
    }
  }
  {
    const v = (dto as any)['exec-trace-config']
    if (v === undefined) {
    } else {
      out['execTraceConfig'] = SimulateTraceConfigModel.fromMsgpackDto(v)
    }
  }
  {
    const v = (dto as any)['fix-signers']
    if (v === undefined) {
    } else {
      out['fixSigners'] = v
    }
  }
  return out as SimulateRequest
}

export const SimulateRequest = {
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
