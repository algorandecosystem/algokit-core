import type { StateProofVerifier } from "./index";

export type StateProofParticipant = { verifier?: StateProofVerifier; weight?: bigint };
