import type { TransactionSignatureMultisigSubsignature } from "./index";

/**
 * \[msig\] structure holding multiple subsignatures.
 *
 * Definition:
 * crypto/multisig.go : MultisigSig
 */
export type TransactionSignatureMultisig = {
  subsignature?: TransactionSignatureMultisigSubsignature[];
  threshold?: bigint;
  version?: bigint;
};
