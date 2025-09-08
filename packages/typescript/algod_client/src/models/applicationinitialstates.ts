import type { ApplicationKvstorage } from "./index";

/**
 * An application's initial global/local/box states that were accessed during simulation.
 */
export type ApplicationInitialStates = {
  id: bigint;
  appLocals?: ApplicationKvstorage[];
  appGlobals?: ApplicationKvstorage;
  appBoxes?: ApplicationKvstorage;
};
