import type { TransactionSignatureLogicsig, TransactionSignatureMultisig } from "./index";

/**
 * Validation signature associated with some data. Only one of the signatures should be provided.
 */
export type TransactionSignature = {
  logicsig?: TransactionSignatureLogicsig;
  multisig?: TransactionSignatureMultisig;

  /**
   * \[sig\] Standard ed25519 signature.
   */
  sig?: string;
};
