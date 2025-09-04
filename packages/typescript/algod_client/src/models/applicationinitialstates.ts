import type { ApplicationKvstorage } from "./index";

/**
 * An application's initial global/local/box states that were accessed during simulation.
 */
export type ApplicationInitialStates = {
  id: bigint;
  "app-locals"?: ApplicationKvstorage[];
  "app-globals"?: ApplicationKvstorage;
  "app-boxes"?: ApplicationKvstorage;
};
