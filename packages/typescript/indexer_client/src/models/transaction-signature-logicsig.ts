import type { TransactionSignatureMultisig } from './index'
/**
 * \[lsig\] Programatic transaction signature.
 *
 * Definition:
 * data/transactions/logicsig.go
 */
export type TransactionSignatureLogicsig = {
  /**
   * \[arg\] Logic arguments, base64 encoded.
   */
  args?: string[]

  /**
   * \[l\] Program signed by a signature or multi signature, or hashed to be the address of an account. Base64 encoded TEAL program.
   */
  logic: string
  multisigSignature?: TransactionSignatureMultisig
  logicMultisigSignature?: TransactionSignatureMultisig

  /**
   * \[sig\] ed25519 signature.
   */
  signature?: string
}
