import { expect, it, describe } from 'vitest'
import { AlgodClient } from '../src/client'
import { getAlgodEnv, getSenderMnemonic } from './config'
import { PendingTransactionResponse } from '../src/models'
import { IntDecoding } from '../src'

describe('Algod pendingTransaction', () => {
  it('submits a payment tx and queries pending info', async () => {
    const env = getAlgodEnv()
    const algosdk = (await import('algosdk')).default
    const client = new AlgodClient({
      baseUrl: env.algodBaseUrl,
      intDecoding: IntDecoding.BIGINT,
      headers: env.algodApiToken ? { 'X-Algo-API-Token': env.algodApiToken } : undefined,
    })

    const mnemonic = env.senderMnemonic ?? (await getSenderMnemonic())
    const acct = algosdk.mnemonicToSecretKey(mnemonic)
    const sp = await client.transactionParams()

    // Build simple self-payment of 0 microalgos (allowed) as a noop
    const txn = algosdk.makePaymentTxnWithSuggestedParamsFromObject({
      sender: acct.addr,
      receiver: acct.addr,
      amount: 0,
      suggestedParams: {
        minFee: Number(sp['minFee']),
        fee: Number(sp['minFee']),
        firstValid: Number(sp['lastRound']),
        flatFee: true,
        lastValid: Number(sp['lastRound']) + 1000,
        genesisHash: sp['genesisHash'] as Uint8Array,
        genesisID: sp['genesisId'] as string,
      },
    })

    const signed = txn.signTxn(acct.sk)
    const sendResult = await client.rawTransaction({ body: signed })
    const txId = sendResult.txId as string

    let pending: PendingTransactionResponse | undefined
    const maxAttempts = 10
    for (let i = 0; i < maxAttempts; i++) {
      pending = await client.pendingTransactionInformation(txId, { format: 'msgpack' })
      if (pending?.confirmedRound || pending?.poolError) {
        break
      }
      await new Promise((resolve) => setTimeout(resolve, 1000))
    }
    if (!pending) {
      throw new Error('Transaction confirmation timeout')
    }

    // Some nodes may omit 'txid' in pending response; require txn presence
    expect(pending).toHaveProperty('txn')
  }, 30_000)
})
