import { expect, it, describe } from 'vitest'
import { IndexerClient } from '../src/client'
import { createDummyAsset, getIndexerEnv } from './config'

describe('Indexer search transactions', () => {
  it('should search for transactions', async () => {
    const { assetId } = await createDummyAsset()
    const env = getIndexerEnv()
    const client = new IndexerClient({ baseUrl: env.indexerBaseUrl, headers: { 'X-Algo-API-Token': env.indexerApiToken ?? '' } })

    const res = await client.searchForTransactions()
    expect(res).toHaveProperty('transactions')
    expect(res.transactions && res.transactions.length).toBeGreaterThan(0)

    const assetTxns = await client.searchForTransactions({ txType: 'acfg', assetId: assetId })
    expect(assetTxns).toHaveProperty('transactions')
    expect(assetTxns.transactions[0].createdAssetIndex).toBeGreaterThan(Number(assetId))
  })
})
