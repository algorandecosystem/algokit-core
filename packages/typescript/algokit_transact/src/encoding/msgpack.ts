import { encode as msgpackEncode, decode as msgpackDecode } from 'algorand-msgpack'
// import { decode as msgpackDecode } from '@msgpack/msgpack'

export function encodeMsgpack<T>(data: T): Uint8Array {
  return new Uint8Array(msgpackEncode(data, { sortKeys: true }))
}

export function decodeMsgpack<T>(encoded: Uint8Array): T {
  return msgpackDecode(encoded) as T
}
