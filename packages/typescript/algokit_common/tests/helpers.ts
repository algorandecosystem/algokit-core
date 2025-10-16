import {
  type Transaction,
  type SignedTransaction,
  TransactionType,
  OnApplicationComplete,
  encodeTransaction,
  encodeSignedTransaction,
  getTransactionId,
  groupTransactions as groupTxns,
} from '@algorandfoundation/algokit-transact'
import { KmdClient } from '@algorandfoundation/kmd-client'
import { AlgodClient, PendingTransactionResponse } from '@algorandfoundation/algod-client'
import algosdk from 'algosdk'
import * as ed from '@noble/ed25519'
import { Buffer } from 'node:buffer'

export interface AlgodTestConfig {
  algodBaseUrl: string
  algodApiToken?: string
  senderMnemonic?: string
}

export function getAlgodEnv(): AlgodTestConfig {
  return {
    algodBaseUrl: process.env.ALGOD_BASE_URL ?? 'http://localhost:4001',
    // Default token for localnet (Algorand sandbox / Algokit LocalNet)
    algodApiToken: process.env.ALGOD_API_TOKEN ?? 'a'.repeat(64),
    senderMnemonic: process.env.SENDER_MNEMONIC,
  }
}

export async function waitForConfirmation(algod: AlgodClient, txId: string, attempts = 30): Promise<PendingTransactionResponse> {
  for (let i = 0; i < attempts; i++) {
    const pending = await algod.pendingTransactionInformation(txId)
    if (pending.confirmedRound !== undefined && pending.confirmedRound > 0n) {
      return pending
    }
    await new Promise((resolve) => setTimeout(resolve, 500))
  }
  throw new Error(`Transaction ${txId} unconfirmed after ${attempts} attempts`)
}

export async function getSenderMnemonic(): Promise<string> {
  if (process.env.SENDER_MNEMONIC) return process.env.SENDER_MNEMONIC
  const kmdBase = process.env.KMD_BASE_URL ?? 'http://localhost:4002'
  const kmdToken = process.env.KMD_API_TOKEN ?? 'a'.repeat(64)
  const walletPassword = process.env.KMD_WALLET_PASSWORD ?? ''
  const preferredWalletName = process.env.KMD_WALLET_NAME ?? 'unencrypted-default-wallet'

  const kmd = new KmdClient({
    baseUrl: kmdBase,
    apiToken: kmdToken,
  })

  const walletsResponse = await kmd.listWallets()
  const wallets = walletsResponse.wallets ?? []
  if (wallets.length === 0) {
    throw new Error('No KMD wallets available')
  }

  const wallet = wallets.find((w) => (w.name ?? '').toLowerCase() === preferredWalletName.toLowerCase()) ?? wallets[0]

  const walletId = wallet.id
  if (!walletId) {
    throw new Error('Wallet returned from KMD does not have an id')
  }

  const handleResponse = await kmd.initWalletHandleToken({
    body: {
      walletId,
      walletPassword,
    },
  })

  const walletHandleToken = handleResponse.walletHandleToken
  if (!walletHandleToken) {
    throw new Error('Failed to obtain wallet handle token from KMD')
  }

  try {
    const keysResponse = await kmd.listKeysInWallet({
      body: {
        walletHandleToken,
      },
    })
    let address = keysResponse.addresses?.[0]
    if (!address) {
      const generated = await kmd.generateKey({
        body: {
          walletHandleToken,
          displayMnemonic: false,
        },
      })
      address = generated.address ?? undefined
    }

    if (!address) {
      throw new Error('Unable to determine or generate a wallet key from KMD')
    }

    const exportResponse = await kmd.exportKey({
      body: {
        walletHandleToken,
        walletPassword,
        address,
      },
    })

    const exportedKey = exportResponse.privateKey
    if (!exportedKey) {
      throw new Error('KMD key export did not return a private key')
    }

    const secretKey = new Uint8Array(exportedKey)
    return algosdk.secretKeyToMnemonic(secretKey)
  } finally {
    await kmd
      .releaseWalletHandleToken({
        body: {
          walletHandleToken,
        },
      })
      .catch(() => undefined)
  }
}

/**
 * Convenience helper: derive the sender account (address + keys) used for tests.
 * Returns:
 *  - address: Algorand address string
 *  - secretKey: 64-byte Ed25519 secret key (private + public)
 *  - mnemonic: the 25-word mnemonic
 */
export async function getSenderAccount(): Promise<{
  address: string
  secretKey: Uint8Array
  mnemonic: string
}> {
  const mnemonic = await getSenderMnemonic()
  const { addr, sk } = algosdk.mnemonicToSecretKey(mnemonic) // TODO: Remove algosdk dependency
  const secretKey = new Uint8Array(sk)
  return { address: typeof addr === 'string' ? addr : addr.toString(), secretKey, mnemonic }
}

export async function signTransaction(transaction: Transaction, secretKey: Uint8Array): Promise<SignedTransaction> {
  const encodedTxn = encodeTransaction(transaction)
  const signature = await ed.signAsync(encodedTxn, secretKey.slice(0, 32))

  return {
    transaction,
    signature,
  }
}

export function groupTransactions(transactions: Transaction[]): Transaction[] {
  return groupTxns(transactions)
}

export interface IndexerTestConfig {
  indexerBaseUrl: string
  indexerApiToken?: string
}

export interface CreatedAssetInfo {
  assetId: bigint
  txId: string
}

export interface CreatedAppInfo {
  appId: bigint
  txId: string
}

function getAlgodClient(): AlgodClient {
  const env = getAlgodEnv()
  return new AlgodClient({ baseUrl: env.algodBaseUrl, apiToken: env.algodApiToken })
}

function decodeGenesisHash(genesisHash: string | Uint8Array): Uint8Array {
  if (genesisHash instanceof Uint8Array) {
    return new Uint8Array(genesisHash)
  }
  return new Uint8Array(Buffer.from(genesisHash, 'base64'))
}

async function submitTransaction(transaction: Transaction, algod: AlgodClient, secretKey: Uint8Array): Promise<{ txId: string }> {
  const signed = await signTransaction(transaction, secretKey)
  const raw = encodeSignedTransaction(signed)
  const txId = getTransactionId(transaction)
  await algod.rawTransaction({ body: raw })
  await waitForConfirmation(algod, txId, 10)
  return { txId }
}

export async function createDummyAsset(): Promise<CreatedAssetInfo> {
  const { address, secretKey } = await getSenderAccount()
  const algod = getAlgodClient()
  const sp = await algod.transactionParams()

  const firstValid = sp.lastRound
  const lastValid = firstValid + 1_000n

  const transaction: Transaction = {
    transactionType: TransactionType.AssetConfig,
    sender: address,
    firstValid,
    lastValid,
    genesisHash: decodeGenesisHash(sp.genesisHash),
    genesisId: sp.genesisId,
    fee: sp.minFee,
    assetConfig: {
      assetId: 0n,
      total: 1_000_000n,
      decimals: 0,
      defaultFrozen: false,
      assetName: 'DummyAsset',
      unitName: 'DUM',
      manager: address,
      reserve: address,
      freeze: address,
      clawback: address,
    },
  }

  const { txId } = await submitTransaction(transaction, algod, secretKey)

  const assetId = (await algod.pendingTransactionInformation(txId)).assetId as bigint | undefined
  if (!assetId) {
    throw new Error('Asset creation transaction confirmed without returning an asset id')
  }

  return { assetId, txId }
}

export async function createDummyApp(): Promise<CreatedAppInfo> {
  const { address, secretKey } = await getSenderAccount()
  const algod = getAlgodClient()
  const sp = await algod.transactionParams()

  const approvalProgramSource = '#pragma version 8\nint 1'
  const clearProgramSource = '#pragma version 8\nint 1'

  const compile = async (source: string) => {
    const result = await algod.tealCompile({ body: source })
    return new Uint8Array(Buffer.from(result.result, 'base64'))
  }

  const approvalProgram = await compile(approvalProgramSource)
  const clearProgram = await compile(clearProgramSource)

  const firstValid = sp.lastRound
  const lastValid = sp.lastRound + 1_000n

  const transaction: Transaction = {
    transactionType: TransactionType.AppCall,
    sender: address,
    firstValid,
    fee: sp.minFee,
    lastValid,
    genesisHash: decodeGenesisHash(sp.genesisHash),
    genesisId: sp.genesisId,
    appCall: {
      appId: 0n,
      onComplete: OnApplicationComplete.NoOp,
      approvalProgram,
      clearStateProgram: clearProgram,
      globalStateSchema: {
        numUints: 1,
        numByteSlices: 1,
      },
      localStateSchema: {
        numUints: 0,
        numByteSlices: 0,
      },
    },
  }

  const { txId } = await submitTransaction(transaction, algod, secretKey)

  const appId = (await algod.pendingTransactionInformation(txId)).appId
  if (!appId) {
    throw new Error('Application creation transaction confirmed without returning an app id')
  }

  return { appId, txId }
}

export function getIndexerEnv(): IndexerTestConfig {
  return {
    indexerBaseUrl: process.env.INDEXER_BASE_URL ?? 'http://localhost:8980',
    indexerApiToken: process.env.INDEXER_API_TOKEN ?? 'a'.repeat(64),
  }
}
