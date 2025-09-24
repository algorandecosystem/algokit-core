import type {
  AccountStateDelta,
  StateDelta,
  TransactionApplication,
  TransactionAssetConfig,
  TransactionAssetFreeze,
  TransactionAssetTransfer,
  TransactionHeartbeat,
  TransactionKeyreg,
  TransactionPayment,
  TransactionSignature,
  TransactionStateProof,
} from './index'
/**
 * Contains all fields common to all transactions and serves as an envelope to all transactions type. Represents both regular and inner transactions.
 *
 * Definition:
 * data/transactions/signedtxn.go : SignedTxn
 * data/transactions/transaction.go : Transaction
 */
export type Transaction = {
  applicationTransaction?: TransactionApplication
  assetConfigTransaction?: TransactionAssetConfig
  assetFreezeTransaction?: TransactionAssetFreeze
  assetTransferTransaction?: TransactionAssetTransfer
  stateProofTransaction?: TransactionStateProof
  heartbeatTransaction?: TransactionHeartbeat

  /**
   * \[sgnr\] this is included with signed transactions when the signing address does not equal the sender. The backend can use this to ensure that auth addr is equal to the accounts auth addr.
   */
  authAddr?: string

  /**
   * \[rc\] rewards applied to close-remainder-to account.
   */
  closeRewards?: bigint

  /**
   * \[ca\] closing amount for transaction.
   */
  closingAmount?: bigint

  /**
   * Round when the transaction was confirmed.
   */
  confirmedRound?: bigint

  /**
   * Specifies an application index (ID) if an application was created with this transaction.
   */
  createdApplicationIndex?: bigint

  /**
   * Specifies an asset index (ID) if an asset was created with this transaction.
   */
  createdAssetIndex?: bigint

  /**
   * \[fee\] Transaction fee.
   */
  fee: bigint

  /**
   * \[fv\] First valid round for this transaction.
   */
  firstValid: bigint

  /**
   * \[gh\] Hash of genesis block.
   */
  genesisHash?: Uint8Array

  /**
   * \[gen\] genesis block ID.
   */
  genesisId?: string

  /**
   * \[grp\] Base64 encoded byte array of a sha512/256 digest. When present indicates that this transaction is part of a transaction group and the value is the sha512/256 hash of the transactions in that group.
   */
  group?: Uint8Array

  /**
   * Transaction ID
   */
  id?: string

  /**
   * Offset into the round where this transaction was confirmed.
   */
  intraRoundOffset?: bigint
  keyregTransaction?: TransactionKeyreg

  /**
   * \[lv\] Last valid round for this transaction.
   */
  lastValid: bigint

  /**
   * \[lx\] Base64 encoded 32-byte array. Lease enforces mutual exclusion of transactions.  If this field is nonzero, then once the transaction is confirmed, it acquires the lease identified by the (Sender, Lease) pair of the transaction until the LastValid round passes.  While this transaction possesses the lease, no other transaction specifying this lease can be confirmed.
   */
  lease?: Uint8Array

  /**
   * \[note\] Free form data.
   */
  note?: Uint8Array
  paymentTransaction?: TransactionPayment

  /**
   * \[rr\] rewards applied to receiver account.
   */
  receiverRewards?: bigint

  /**
   * \[rekey\] when included in a valid transaction, the accounts auth addr will be updated with this value and future signatures must be signed with the key represented by this address.
   */
  rekeyTo?: string

  /**
   * Time when the block this transaction is in was confirmed.
   */
  roundTime?: bigint

  /**
   * \[snd\] Sender's address.
   */
  sender: string

  /**
   * \[rs\] rewards applied to sender account.
   */
  senderRewards?: bigint
  signature?: TransactionSignature

  /**
   * \[type\] Indicates what type of transaction this is. Different types have different fields.
   *
   * Valid types, and where their fields are stored:
   * * \[pay\] payment-transaction
   * * \[keyreg\] keyreg-transaction
   * * \[acfg\] asset-config-transaction
   * * \[axfer\] asset-transfer-transaction
   * * \[afrz\] asset-freeze-transaction
   * * \[appl\] application-transaction
   * * \[stpf\] state-proof-transaction
   * * \[hb\] heartbeat-transaction
   */
  txType: 'pay' | 'keyreg' | 'acfg' | 'axfer' | 'afrz' | 'appl' | 'stpf' | 'hb'

  /**
   * \[ld\] Local state key/value changes for the application being executed by this transaction.
   */
  localStateDelta?: AccountStateDelta[]
  globalStateDelta?: StateDelta

  /**
   * \[lg\] Logs for the application being executed by this transaction.
   */
  logs?: Uint8Array[]

  /**
   * Inner transactions produced by application execution.
   */
  innerTxns?: Transaction[]
}
