import { expect, it, describe } from 'vitest'
import { IndexerClient } from '../src/client'
import { getIndexerEnv } from './config'

describe('Indexer search transactions', () => {
  it('should search for transactions', async () => {
    const env = getIndexerEnv()
    const client = new IndexerClient({ baseUrl: env.indexerBaseUrl, headers: { 'X-Algo-API-Token': env.indexerApiToken ?? '' } })
    const res = await client.searchForTransactions()
    expect(res).toHaveProperty('transactions')
  })
})
