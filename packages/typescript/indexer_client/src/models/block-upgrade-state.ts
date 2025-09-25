import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Fields relating to a protocol upgrade.
 */
export type BlockUpgradeState = {
  /**
   * \[proto\] The current protocol version.
   */
  currentProtocol: string

  /**
   * \[nextproto\] The next proposed protocol version.
   */
  nextProtocol?: string

  /**
   * \[nextyes\] Number of blocks which approved the protocol upgrade.
   */
  nextProtocolApprovals?: bigint

  /**
   * \[nextswitch\] Round on which the protocol upgrade will take effect.
   */
  nextProtocolSwitchOn?: bigint

  /**
   * \[nextbefore\] Deadline round for this protocol upgrade (No votes will be consider after this round).
   */
  nextProtocolVoteBefore?: bigint
}

// JSON DTO shape for BlockUpgradeState with wire keys and JSON-safe primitives
export type BlockUpgradeStateDto = {
  'current-protocol': string
  'next-protocol'?: string
  'next-protocol-approvals'?: bigint
  'next-protocol-switch-on'?: bigint
  'next-protocol-vote-before'?: bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: BlockUpgradeState): BlockUpgradeStateDto {
  const out: any = {}
  {
    const v = (value as any)['currentProtocol']
    if (v === undefined) {
      // omit undefined
    } else {
      out['current-protocol'] = v
    }
  }
  {
    const v = (value as any)['nextProtocol']
    if (v === undefined) {
      // omit undefined
    } else {
      out['next-protocol'] = v
    }
  }
  {
    const v = (value as any)['nextProtocolApprovals']
    if (v === undefined) {
      // omit undefined
    } else {
      out['next-protocol-approvals'] = v
    }
  }
  {
    const v = (value as any)['nextProtocolSwitchOn']
    if (v === undefined) {
      // omit undefined
    } else {
      out['next-protocol-switch-on'] = v
    }
  }
  {
    const v = (value as any)['nextProtocolVoteBefore']
    if (v === undefined) {
      // omit undefined
    } else {
      out['next-protocol-vote-before'] = v
    }
  }
  return out as BlockUpgradeStateDto
}

export function fromDto(dto: BlockUpgradeStateDto): BlockUpgradeState {
  const out: any = {}
  {
    const v = (dto as any)['current-protocol']
    if (v === undefined) {
      // omit undefined
    } else {
      out['currentProtocol'] = v as any
    }
  }
  {
    const v = (dto as any)['next-protocol']
    if (v === undefined) {
      // omit undefined
    } else {
      out['nextProtocol'] = v as any
    }
  }
  {
    const v = (dto as any)['next-protocol-approvals']
    if (v === undefined) {
      // omit undefined
    } else {
      out['nextProtocolApprovals'] = v as any
    }
  }
  {
    const v = (dto as any)['next-protocol-switch-on']
    if (v === undefined) {
      // omit undefined
    } else {
      out['nextProtocolSwitchOn'] = v as any
    }
  }
  {
    const v = (dto as any)['next-protocol-vote-before']
    if (v === undefined) {
      // omit undefined
    } else {
      out['nextProtocolVoteBefore'] = v as any
    }
  }
  return out as BlockUpgradeState
}

// Msgpack codecs
export function encodeMsgpack(value: BlockUpgradeState): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): BlockUpgradeState {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: BlockUpgradeState): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): BlockUpgradeState {
  return fromDto(raw as BlockUpgradeStateDto)
}

// Array helpers
export function encodeMsgpackArray(values: BlockUpgradeState[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): BlockUpgradeState[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: BlockUpgradeState[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): BlockUpgradeState[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type BlockUpgradeStateMsgpackDto = {
  'current-protocol': string
  'next-protocol'?: string
  'next-protocol-approvals'?: bigint
  'next-protocol-switch-on'?: bigint
  'next-protocol-vote-before'?: bigint
}

function toMsgpackDto(value: BlockUpgradeState): BlockUpgradeStateMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['currentProtocol']
    if (v === undefined) {
    } else {
      out['current-protocol'] = v
    }
  }
  {
    const v = (value as any)['nextProtocol']
    if (v === undefined) {
    } else {
      out['next-protocol'] = v
    }
  }
  {
    const v = (value as any)['nextProtocolApprovals']
    if (v === undefined) {
    } else {
      out['next-protocol-approvals'] = v
    }
  }
  {
    const v = (value as any)['nextProtocolSwitchOn']
    if (v === undefined) {
    } else {
      out['next-protocol-switch-on'] = v
    }
  }
  {
    const v = (value as any)['nextProtocolVoteBefore']
    if (v === undefined) {
    } else {
      out['next-protocol-vote-before'] = v
    }
  }
  return out as BlockUpgradeStateMsgpackDto
}

function fromMsgpackDto(dto: BlockUpgradeStateMsgpackDto): BlockUpgradeState {
  const out: any = {}
  {
    const v = (dto as any)['current-protocol']
    if (v === undefined) {
    } else {
      out['currentProtocol'] = v
    }
  }
  {
    const v = (dto as any)['next-protocol']
    if (v === undefined) {
    } else {
      out['nextProtocol'] = v
    }
  }
  {
    const v = (dto as any)['next-protocol-approvals']
    if (v === undefined) {
    } else {
      out['nextProtocolApprovals'] = v
    }
  }
  {
    const v = (dto as any)['next-protocol-switch-on']
    if (v === undefined) {
    } else {
      out['nextProtocolSwitchOn'] = v
    }
  }
  {
    const v = (dto as any)['next-protocol-vote-before']
    if (v === undefined) {
    } else {
      out['nextProtocolVoteBefore'] = v
    }
  }
  return out as BlockUpgradeState
}

export const BlockUpgradeState = {
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
