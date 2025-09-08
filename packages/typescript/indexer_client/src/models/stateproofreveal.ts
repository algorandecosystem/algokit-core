import type { StateProofParticipant, StateProofSigSlot } from "./index";

export type StateProofReveal = { position?: bigint; sigSlot?: StateProofSigSlot; participant?: StateProofParticipant };
