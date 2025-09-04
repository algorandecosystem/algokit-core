import type { StateProofParticipant, StateProofSigSlot } from "./index";

export type StateProofReveal = { position?: bigint; "sig-slot"?: StateProofSigSlot; participant?: StateProofParticipant };
