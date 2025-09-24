import { describe } from 'vitest'

export interface IndexerTestConfig {
  indexerBaseUrl: string
  indexerApiToken?: string
}

export interface CreatedAssetInfo {
  assetId: number
  txId: string
}

export interface CreatedAppInfo {
  appId: number
  txId: string
}

export async function getSenderMnemonic(): Promise<string> {
  if (process.env.SENDER_MNEMONIC) return process.env.SENDER_MNEMONIC
  const algosdk = (await import('algosdk')).default
  // Try to derive from local KMD defaults
  const kmdBase = process.env.KMD_BASE_URL ?? 'http://localhost:4002'
  const kmdToken = process.env.KMD_API_TOKEN ?? 'a'.repeat(64)
  const url = new URL(kmdBase)
  const server = `${url.protocol}//${url.hostname}`
  const port = Number(url.port || 4002)
  const kmd = new algosdk.Kmd(kmdToken, server, port)
  const wallets = await kmd.listWallets()
  const wallet = wallets.wallets.find((w: any) => w.name === 'unencrypted-default-wallet') ?? wallets.wallets[0]
  if (!wallet) throw new Error('No KMD wallet found on localnet')
  const handle = await kmd.initWalletHandle(wallet.id, '')
  try {
    const keys = await kmd.listKeys(handle.wallet_handle_token)
    let address: string | undefined = keys.addresses[0]
    if (!address) {
      const gen = await kmd.generateKey(handle.wallet_handle_token)
      address = gen.address
    }
    const exported = await kmd.exportKey(handle.wallet_handle_token, '', address!)
    const sk = new Uint8Array(exported.private_key)
    return algosdk.secretKeyToMnemonic(sk)
  } finally {
    await kmd.releaseWalletHandle(handle.wallet_handle_token)
  }
}

export async function createDummyAsset(): Promise<CreatedAssetInfo> {
  const algosdk = (await import('algosdk')).default
  const mnemonic = await getSenderMnemonic()
  const { addr, sk } = algosdk.mnemonicToSecretKey(mnemonic)

  const algod = new algosdk.Algodv2('a'.repeat(64), 'http://localhost', 4001)
  const sp = await algod.getTransactionParams().do()

  const txn = algosdk.makeAssetCreateTxnWithSuggestedParamsFromObject({
    sender: addr,
    assetName: 'DummyAsset',
    unitName: 'DUM',
    total: 1_000_000,
    decimals: 0,
    defaultFrozen: false,
    manager: addr,
    reserve: addr,
    freeze: addr,
    clawback: addr,
    suggestedParams: sp,
  })

  const signed = txn.signTxn(sk)
  await algod.sendRawTransaction(signed).do()
  const txId = txn.txID()
  const pending = await algosdk.waitForConfirmation(algod, txId, 10)
  const assetId = pending['asset-index'] as number
  return { assetId, txId }
}

export async function createDummyApp(): Promise<CreatedAppInfo> {
  const algosdk = (await import('algosdk')).default
  const mnemonic = await getSenderMnemonic()
  const { addr, sk } = algosdk.mnemonicToSecretKey(mnemonic)

  const algod = new algosdk.Algodv2('a'.repeat(64), 'http://localhost', 4001)
  const sp = await algod.getTransactionParams().do()

  const approvalProgramSource = '#pragma version 8\nint 1'
  const clearProgramSource = '#pragma version 8\nint 1'

  const compile = async (source: string) => {
    const res = await algod.compile(source).do()
    return new Uint8Array(Buffer.from(res.result, 'base64'))
  }

  const approvalProgram = await compile(approvalProgramSource)
  const clearProgram = await compile(clearProgramSource)

  const txn = algosdk.makeApplicationCreateTxnFromObject({
    sender: addr,
    approvalProgram,
    clearProgram,
    numLocalInts: 0,
    numLocalByteSlices: 0,
    numGlobalInts: 1,
    numGlobalByteSlices: 1,
    onComplete: algosdk.OnApplicationComplete.NoOpOC,
    suggestedParams: sp,
  })

  const signed = txn.signTxn(sk)
  await algod.sendRawTransaction(signed).do()
  const txId = txn.txID()
  const pending = await algosdk.waitForConfirmation(algod, txId, 10)
  const appId = pending['application-index'] as number
  return { appId, txId }
}

export function getIndexerEnv(): IndexerTestConfig {
  return {
    indexerBaseUrl: process.env.INDEXER_BASE_URL ?? 'http://localhost:8980',
    indexerApiToken: process.env.INDEXER_API_TOKEN ?? 'a'.repeat(64),
  }
}

export function maybeDescribe(name: string, fn: (env: IndexerTestConfig) => void) {
  describe(name, () => fn(getIndexerEnv()))
}
