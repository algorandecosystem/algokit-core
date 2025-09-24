import { expect, it, describe } from 'vitest'
import { AlgodClient } from '../src/client'
import { getAlgodEnv } from './config'

describe('transactionParams', () => {
  it('should fetch transaction params', async () => {
    const env = getAlgodEnv()
    const client = new AlgodClient({
      baseUrl: env.algodBaseUrl,
      headers: { 'X-Algo-API-Token': env.algodApiToken ?? '' },
      intDecoding: 'bigint',
    })
    const sp = await client.transactionParams()
    expect(sp).toHaveProperty('genesisHash')
    expect(sp).toHaveProperty('lastRound')
  })
})
