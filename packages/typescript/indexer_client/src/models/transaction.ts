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
} from "./index";

/**
 * Contains all fields common to all transactions and serves as an envelope to all transactions type. Represents both regular and inner transactions.
 *
 * Definition:
 * data/transactions/signedtxn.go : SignedTxn
 * data/transactions/transaction.go : Transaction
 */
export type Transaction = {
  applicationTransaction?: TransactionApplication;
  assetConfigTransaction?: TransactionAssetConfig;
  assetFreezeTransaction?: TransactionAssetFreeze;
  assetTransferTransaction?: TransactionAssetTransfer;
  stateProofTransaction?: TransactionStateProof;
  heartbeatTransaction?: TransactionHeartbeat;
  authAddr?: string;
  closeRewards?: bigint;
  closingAmount?: bigint;
  confirmedRound?: bigint;
  createdApplicationIndex?: bigint;
  createdAssetIndex?: bigint;
  fee: bigint;
  firstValid: bigint;
  genesisHash?: string;
  genesisId?: string;
  group?: string;
  id?: string;
  intraRoundOffset?: bigint;
  keyregTransaction?: TransactionKeyreg;
  lastValid: bigint;
  lease?: string;
  note?: string;
  paymentTransaction?: TransactionPayment;
  receiverRewards?: bigint;
  rekeyTo?: string;
  roundTime?: bigint;
  sender: string;
  senderRewards?: bigint;
  signature?: TransactionSignature;
  txType: "pay" | "keyreg" | "acfg" | "axfer" | "afrz" | "appl" | "stpf" | "hb";
  localStateDelta?: AccountStateDelta[];
  globalStateDelta?: StateDelta;
  logs?: string[];
  innerTxns?: Transaction[];
};
