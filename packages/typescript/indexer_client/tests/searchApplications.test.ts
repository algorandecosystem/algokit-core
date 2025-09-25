import { expect, it, describe } from 'vitest'
import { IndexerClient } from '../src/client'
import { createDummyApp, getIndexerEnv } from './config'

describe('Indexer search applications', () => {
  it('should search for applications', async () => {
    await createDummyApp()
    const env = getIndexerEnv()
    const client = new IndexerClient({ baseUrl: env.indexerBaseUrl, headers: { 'X-Algo-API-Token': env.indexerApiToken ?? '' } })
    const res = await client.searchForApplications()
    expect(res).toHaveProperty('applications')
    expect(res.applications && res.applications.length).toBeGreaterThan(0)
  })
})
