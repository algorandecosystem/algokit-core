import { encodeMsgPack, decodeMsgPack } from '../core/msgpack'
import { toBase64 as _toBase64, fromBase64 as _fromBase64 } from '../core/json'

/**
 * Fields relating to rewards,
 */
export type BlockRewards = {
  /**
   * \[fees\] accepts transaction fees, it can only spend to the incentive pool.
   */
  feeSink: string

  /**
   * \[rwcalr\] number of leftover MicroAlgos after the distribution of rewards-rate MicroAlgos for every reward unit in the next round.
   */
  rewardsCalculationRound: bigint

  /**
   * \[earn\] How many rewards, in MicroAlgos, have been distributed to each RewardUnit of MicroAlgos since genesis.
   */
  rewardsLevel: bigint

  /**
   * \[rwd\] accepts periodic injections from the fee-sink and continually redistributes them as rewards.
   */
  rewardsPool: string

  /**
   * \[rate\] Number of new MicroAlgos added to the participation stake from rewards at the next round.
   */
  rewardsRate: bigint

  /**
   * \[frac\] Number of leftover MicroAlgos after the distribution of RewardsRate/rewardUnits MicroAlgos for every reward unit in the next round.
   */
  rewardsResidue: bigint
}

// JSON DTO shape for BlockRewards with wire keys and JSON-safe primitives
export type BlockRewardsDto = {
  'fee-sink': string
  'rewards-calculation-round': bigint
  'rewards-level': bigint
  'rewards-pool': string
  'rewards-rate': bigint
  'rewards-residue': bigint
}

// Helpers
const toBase64 = _toBase64
const fromBase64 = _fromBase64

// toDto/fromDto
export function toDto(value: BlockRewards): BlockRewardsDto {
  const out: any = {}
  {
    const v = (value as any)['feeSink']
    if (v === undefined) {
      // omit undefined
    } else {
      out['fee-sink'] = v
    }
  }
  {
    const v = (value as any)['rewardsCalculationRound']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewards-calculation-round'] = v
    }
  }
  {
    const v = (value as any)['rewardsLevel']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewards-level'] = v
    }
  }
  {
    const v = (value as any)['rewardsPool']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewards-pool'] = v
    }
  }
  {
    const v = (value as any)['rewardsRate']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewards-rate'] = v
    }
  }
  {
    const v = (value as any)['rewardsResidue']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewards-residue'] = v
    }
  }
  return out as BlockRewardsDto
}

export function fromDto(dto: BlockRewardsDto): BlockRewards {
  const out: any = {}
  {
    const v = (dto as any)['fee-sink']
    if (v === undefined) {
      // omit undefined
    } else {
      out['feeSink'] = v as any
    }
  }
  {
    const v = (dto as any)['rewards-calculation-round']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewardsCalculationRound'] = v as any
    }
  }
  {
    const v = (dto as any)['rewards-level']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewardsLevel'] = v as any
    }
  }
  {
    const v = (dto as any)['rewards-pool']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewardsPool'] = v as any
    }
  }
  {
    const v = (dto as any)['rewards-rate']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewardsRate'] = v as any
    }
  }
  {
    const v = (dto as any)['rewards-residue']
    if (v === undefined) {
      // omit undefined
    } else {
      out['rewardsResidue'] = v as any
    }
  }
  return out as BlockRewards
}

// Msgpack codecs
export function encodeMsgpack(value: BlockRewards): Uint8Array {
  const dto = toMsgpackDto(value)
  return encodeMsgPack(dto)
}

export function decodeMsgpack(bytes: Uint8Array): BlockRewards {
  const raw: any = decodeMsgPack(bytes)
  // raw has wire keys and Uint8Array for bytes
  return fromMsgpackDto(raw)
}

// JSON codecs
export function encodeJson(value: BlockRewards): unknown {
  return toDto(value)
}

export function decodeJson(raw: unknown): BlockRewards {
  return fromDto(raw as BlockRewardsDto)
}

// Array helpers
export function encodeMsgpackArray(values: BlockRewards[]): Uint8Array {
  const dto = values.map((v) => toMsgpackDto(v))
  return encodeMsgPack(dto)
}

export function decodeMsgpackArray(bytes: Uint8Array): BlockRewards[] {
  const raw: any = decodeMsgPack(bytes)
  return (raw as any[]).map((item) => fromMsgpackDto(item))
}

export function encodeJsonArray(values: BlockRewards[]): unknown {
  return values.map((v) => toDto(v))
}

export function decodeJsonArray(raw: unknown): BlockRewards[] {
  return (raw as any[]).map((item) => fromDto(item))
}

// Internal: msgpack DTO (wire keys, bytes kept as Uint8Array, signed txn encoded to bytes)
type BlockRewardsMsgpackDto = {
  'fee-sink': string
  'rewards-calculation-round': bigint
  'rewards-level': bigint
  'rewards-pool': string
  'rewards-rate': bigint
  'rewards-residue': bigint
}

function toMsgpackDto(value: BlockRewards): BlockRewardsMsgpackDto {
  const out: any = {}
  {
    const v = (value as any)['feeSink']
    if (v === undefined) {
    } else {
      out['fee-sink'] = v
    }
  }
  {
    const v = (value as any)['rewardsCalculationRound']
    if (v === undefined) {
    } else {
      out['rewards-calculation-round'] = v
    }
  }
  {
    const v = (value as any)['rewardsLevel']
    if (v === undefined) {
    } else {
      out['rewards-level'] = v
    }
  }
  {
    const v = (value as any)['rewardsPool']
    if (v === undefined) {
    } else {
      out['rewards-pool'] = v
    }
  }
  {
    const v = (value as any)['rewardsRate']
    if (v === undefined) {
    } else {
      out['rewards-rate'] = v
    }
  }
  {
    const v = (value as any)['rewardsResidue']
    if (v === undefined) {
    } else {
      out['rewards-residue'] = v
    }
  }
  return out as BlockRewardsMsgpackDto
}

function fromMsgpackDto(dto: BlockRewardsMsgpackDto): BlockRewards {
  const out: any = {}
  {
    const v = (dto as any)['fee-sink']
    if (v === undefined) {
    } else {
      out['feeSink'] = v
    }
  }
  {
    const v = (dto as any)['rewards-calculation-round']
    if (v === undefined) {
    } else {
      out['rewardsCalculationRound'] = v
    }
  }
  {
    const v = (dto as any)['rewards-level']
    if (v === undefined) {
    } else {
      out['rewardsLevel'] = v
    }
  }
  {
    const v = (dto as any)['rewards-pool']
    if (v === undefined) {
    } else {
      out['rewardsPool'] = v
    }
  }
  {
    const v = (dto as any)['rewards-rate']
    if (v === undefined) {
    } else {
      out['rewardsRate'] = v
    }
  }
  {
    const v = (dto as any)['rewards-residue']
    if (v === undefined) {
    } else {
      out['rewardsResidue'] = v
    }
  }
  return out as BlockRewards
}

export const BlockRewards = {
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
