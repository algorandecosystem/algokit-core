import type { AlgodClient } from '@algorandfoundation/algod-client'
import { signTransaction as signTransactionHelper, getSenderAccount } from '../../../algokit_common/tests/helpers'
import { AssetManager } from '../../src/clients/asset-manager'
import { ClientManager } from '../../src/clients/client-manager'
import { TransactionComposer } from '../../src/transactions/composer'
import type { SignerGetter, TransactionSigner } from '../../src/transactions/common'
import type { AssetCreateParams } from '../../src/transactions/asset-config'
import type { AssetTransferParams } from '../../src/transactions/asset-transfer'
import type { PaymentParams } from '../../src/transactions/payment'
import type { Transaction, SignedTransaction } from '@algorandfoundation/algokit-transact'
import type { PendingTransactionResponse } from '@algorandfoundation/algod-client'
import algosdk from 'algosdk'
import { randomBytes } from 'crypto'

export type TestAccount = {
  address: string
  secretKey: Uint8Array
}

type AssetTestContext = {
  algodClient: AlgodClient
  assetManager: AssetManager
  creator: TestAccount
  signers: InMemorySignerRegistry
  newComposer: () => TransactionComposer
}

class InMemorySignerRegistry implements SignerGetter {
  private readonly signers = new Map<string, TransactionSigner>()
  private readonly secrets = new Map<string, Uint8Array>()
  private defaultSigner?: TransactionSigner
  private defaultSecret?: Uint8Array

  register(address: string, secretKey: Uint8Array): void {
    this.secrets.set(address, secretKey)
    this.signers.set(address, createTransactionSigner(secretKey))
  }

  setDefault(address: string, secretKey: Uint8Array): void {
    this.defaultSecret = secretKey
    this.defaultSigner = createTransactionSigner(secretKey)
    this.register(address, secretKey)
  }

  getSigner(address: string): TransactionSigner {
    const signer = this.signers.get(address) ?? this.defaultSigner
    if (!signer) {
      throw new Error(`No signer registered for address ${address}`)
    }
    return signer
  }

  getSecret(address: string): Uint8Array {
    const secret = this.secrets.get(address) ?? this.defaultSecret
    if (!secret) {
      throw new Error(`No secret key registered for address ${address}`)
    }
    return secret
  }
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

  const signers = new InMemorySignerRegistry()
  signers.setDefault(creator.address, creator.secretKey)

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
  const params: AssetCreateParams = {
    sender: context.creator.address,
    total: overrides.total ?? 1_000n,
    decimals: overrides.decimals ?? 0,
    unitName: overrides.unitName ?? 'TEST',
    assetName: overrides.assetName ?? 'Test Asset',
    defaultFrozen: overrides.defaultFrozen,
    manager: overrides.manager,
    reserve: overrides.reserve,
    freeze: overrides.freeze,
    clawback: overrides.clawback,
    note: new Uint8Array(randomBytes(8)),
  }

  composer.addAssetCreate({ ...params, ...overrides })
  const sendResult = await composer.send({ maxRoundsToWaitForConfirmation: 10 })

  const confirmation = sendResult.results.at(-1)?.confirmation
  if (confirmation?.assetId !== undefined && confirmation.assetId > 0) {
    await waitForConfirmation(context.algodClient, sendResult.results.at(-1)!.transactionId)
    return confirmation.assetId
  }

  const txId = sendResult.results.at(-1)?.transactionId
  if (!txId) {
    throw new Error('Asset creation composer did not return a transaction id')
  }
  const pending = await waitForConfirmation(context.algodClient, txId)
  if (pending.assetId === undefined) {
    throw new Error('Pending transaction response missing assetId')
  }
  return pending.assetId
}

export async function createFundedAccount(
  context: AssetTestContext,
  initialFunding: bigint = 5_000_000n,
): Promise<TestAccount> {
  const generated = algosdk.generateAccount() // TODO: Remove algosdk dependency
  const account: TestAccount = {
    address: generated.addr.toString(),
    secretKey: new Uint8Array(generated.sk),
  }

  context.signers.register(account.address, account.secretKey)
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
  const signSingle = async (transaction: Transaction): Promise<SignedTransaction> => {
    if (!transaction.sender) {
      throw new Error('Transaction missing sender')
    }
    return signTransactionHelper(transaction, secretKey)
  }

  return {
    signTransaction: signSingle,
    signTransactions: async (transactions: Transaction[], indices: number[]) => {
      const signed: SignedTransaction[] = []
      for (const index of indices) {
        const transaction = transactions[index]
        if (!transaction) {
          throw new Error(`Missing transaction at index ${index}`)
        }
        signed.push(await signSingle(transaction))
      }
      return signed
    },
  }
}

async function waitForConfirmation(algod: AlgodClient, txId: string, attempts = 30): Promise<PendingTransactionResponse> {
  for (let i = 0; i < attempts; i++) {
    const pending = await algod.pendingTransactionInformation(txId)
    if (pending.confirmedRound !== undefined && pending.confirmedRound > 0n) {
      return pending
    }
    await delay(500)
  }
  throw new Error(`Transaction ${txId} unconfirmed after ${attempts} attempts`)
}

function delay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms))
}
