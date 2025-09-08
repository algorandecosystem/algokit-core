import type { TransactionSignatureMultisig } from "./index";

/**
 * \[lsig\] Programatic transaction signature.
 *
 * Definition:
 * data/transactions/logicsig.go
 */
export type TransactionSignatureLogicsig = {
  args?: string[];
  logic: string;
  multisigSignature?: TransactionSignatureMultisig;
  signature?: string;
};
