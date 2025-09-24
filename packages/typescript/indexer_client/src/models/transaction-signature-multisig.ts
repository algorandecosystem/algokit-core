import type { TransactionSignatureMultisigSubsignature } from './index'
/**
 * structure holding multiple subsignatures.
 *
 * Definition:
 * crypto/multisig.go : MultisigSig
 */
export type TransactionSignatureMultisig = {
  /**
   * \[subsig\] holds pairs of public key and signatures.
   */
  subsignature?: TransactionSignatureMultisigSubsignature[]

  /**
   * \[thr\]
   */
  threshold?: bigint

  /**
   * \[v\]
   */
  version?: bigint
}
