import type { AlgodClient } from '@algorandfoundation/algod-client'
import { AccountManager } from '../../src/clients/account-manager'
import { AssetManager } from '../../src/clients/asset-manager'
import { ClientManager } from '../../src/clients/client-manager'
import { TransactionComposer } from '../../src/transactions/composer'
import type { TransactionSigner } from '../../src/transactions/common'
import {
  encodeTransaction,
  type AssetCreateParams,
  type AssetTransferParams,
  type PaymentParams,
  type SignedTransaction,
  type Transaction,
} from '@algorandfoundation/algokit-transact'
import * as ed from '@noble/ed25519'
import algosdk from 'algosdk'
import { getSenderAccount } from '../algod/helpers'

export type TestAccount = {
  address: string
  secretKey: Uint8Array
}

type AssetTestContext = {
  algodClient: AlgodClient
  assetManager: AssetManager
  creator: TestAccount
  signers: AccountManager
  newComposer: () => TransactionComposer
}

export function createClientManager(): ClientManager {
  const config = ClientManager.getConfigFromEnvironmentOrLocalNet()
  return new ClientManager(config)
}

export async function createAssetTestContext(): Promise<AssetTestContext> {
  const config = ClientManager.getConfigFromEnvironmentOrLocalNet()
  const algodClient = ClientManager.getAlgodClient(config.algodConfig)
  const creatorAccount = await getSenderAccount()
  const creator: TestAccount = {
    address: creatorAccount.address,
    secretKey: creatorAccount.secretKey,
  }

  const signers = new AccountManager()
  signers.setSigner(creator.address, createTransactionSigner(creator.secretKey))

  const newComposer = () =>
    new TransactionComposer({
      algodClient,
      signerGetter: signers,
    })

  const assetManager = new AssetManager(algodClient, newComposer)

  return {
    algodClient,
    assetManager,
    creator,
    signers,
    newComposer,
  }
}

export async function createTestAsset(
  context: AssetTestContext,
  overrides: Partial<AssetCreateParams> = {},
): Promise<bigint> {
  const composer = context.newComposer()
  const defaultParams: AssetCreateParams = {
    sender: context.creator.address,
    total: overrides.total ?? 1_000n,
    decimals: overrides.decimals ?? 0,
    unitName: overrides.unitName ?? 'TEST',
    assetName: overrides.assetName ?? 'Test Asset',
  }

  composer.addAssetCreate({ ...defaultParams, ...overrides })
  const sendResult = await composer.send({ maxRoundsToWaitForConfirmation: 10 })
  const confirmation = sendResult.results[0]?.confirmation

  if (confirmation?.assetId !== undefined) {
    return confirmation.assetId
  }

  const txId = sendResult.results[0]?.transactionId
  if (!txId) {
    throw new Error('Failed to retrieve transaction id for asset creation')
  }

  const pending = await context.algodClient.pendingTransactionInformation(txId)
  if (pending.assetId === undefined) {
    throw new Error('Pending transaction response did not include asset id')
  }
  return pending.assetId
}

export async function createFundedAccount(
  context: AssetTestContext,
  initialFunding: bigint = 5_000_000n,
): Promise<TestAccount> {
  const generated = algosdk.generateAccount()
  const account: TestAccount = {
    address: generated.addr,
    secretKey: new Uint8Array(generated.sk),
  }

  context.signers.setSigner(account.address, createTransactionSigner(account.secretKey))
  await sendPayment(context, {
    sender: context.creator.address,
    receiver: account.address,
    amount: initialFunding,
  })
  return account
}

export async function sendPayment(context: AssetTestContext, params: PaymentParams) {
  const composer = context.newComposer()
  composer.addPayment(params)
  await composer.send({ maxRoundsToWaitForConfirmation: 10 })
}

export async function transferAsset(context: AssetTestContext, params: AssetTransferParams) {
  const composer = context.newComposer()
  composer.addAssetTransfer(params)
  await composer.send({ maxRoundsToWaitForConfirmation: 10 })
}

function createTransactionSigner(secretKey: Uint8Array): TransactionSigner {
  const privateKey = secretKey.slice(0, 32)

  const signSingle = async (transaction: Transaction): Promise<SignedTransaction> => {
    const bytes = encodeTransaction(transaction)
    const signature = await ed.signAsync(bytes, privateKey)
    return {
      transaction,
      signature,
    }
  }

  return {
    signTransaction: signSingle,
    signTransactions: async (transactions: Transaction[], indices: number[]) => {
      const signed: SignedTransaction[] = []
      for (const index of indices) {
        signed.push(await signSingle(transactions[index]))
      }
      return signed
    },
  }
}
