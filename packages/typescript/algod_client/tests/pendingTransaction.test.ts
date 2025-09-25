import { expect, it, describe } from 'vitest'
import { AlgodClient } from '../src/client'
import { getAlgodEnv, getSenderAccount, signTransaction } from './config'
import { PendingTransactionResponse, PendingTransactionResponseCodecs } from '../src/models'
import { IntDecoding } from '../src'
import { encodeSignedTransaction, getTransactionId, TransactionType, type Transaction } from '@algorandfoundation/algokit-transact'

describe('Algod pendingTransaction', () => {
  it('submits a payment tx and queries pending info', async () => {
    const env = getAlgodEnv()
    const client = new AlgodClient({
      baseUrl: env.algodBaseUrl,
      intDecoding: IntDecoding.BIGINT,
      headers: env.algodApiToken ? { 'X-Algo-API-Token': env.algodApiToken } : undefined,
    })
    const acct = await getSenderAccount()
    const sp = await client.transactionParams()

    const senderAddress = acct.address
    const transaction: Transaction = {
      transactionType: TransactionType.Payment,
      sender: senderAddress,
      fee: BigInt(sp['minFee']), // flat fee
      firstValid: BigInt(sp['lastRound']),
      lastValid: BigInt(sp['lastRound']) + 1000n,
      genesisHash: sp['genesisHash'] as Uint8Array,
      genesisId: sp['genesisId'] as string,
      payment: {
        receiver: senderAddress,
        amount: 0n,
      },
    }

    const signedTransaction = await signTransaction(transaction, acct.secretKey)
    const signedBytes = encodeSignedTransaction(signedTransaction)
    const sendResult = await client.rawTransaction({ body: signedBytes })
    const txId = getTransactionId(transaction)
    expect(sendResult.txId).toBe(txId)

    let pending: PendingTransactionResponse | undefined
    const maxAttempts = 10
    for (let i = 0; i < maxAttempts; i++) {
      const pendingBytes = await client.pendingTransactionInformation(txId, { format: 'msgpack' })
      pending = PendingTransactionResponseCodecs.decodeMsgpack(pendingBytes)
      if (pending?.confirmedRound || pending?.poolError) {
        break
      }
      await new Promise((resolve) => setTimeout(resolve, 1000))
    }
    if (!pending) {
      throw new Error('Transaction confirmation timeout')
    }

    expect(pending).toHaveProperty('txn')
  }, 30_000)
})
