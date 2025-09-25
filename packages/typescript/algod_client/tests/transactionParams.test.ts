import { expect, it, describe } from 'vitest'
import { AlgodClient } from '../src/client'
import { getAlgodEnv } from './config'

describe('transactionParams', () => {
  it('should fetch transaction params', async () => {
    const env = getAlgodEnv()
    const client = new AlgodClient({
      baseUrl: env.algodBaseUrl,
      headers: { 'X-Algo-API-Token': env.algodApiToken ?? '' },
    })
    const sp = await client.transactionParams()
    expect(sp).toHaveProperty('genesisHash')
    expect(sp.genesisHash).toBeInstanceOf(Uint8Array)
    expect(sp).toHaveProperty('lastRound')
  })
})
