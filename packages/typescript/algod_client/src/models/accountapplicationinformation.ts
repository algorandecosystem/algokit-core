import type { ApplicationLocalState, ApplicationParams } from "./index";

/**
 * AccountApplicationResponse describes the account's application local state and global state (AppLocalState and AppParams, if either exists) for a specific application ID. Global state will only be returned if the provided address is the application's creator.
 */
export type AccountApplicationInformation = { round: bigint; appLocalState?: ApplicationLocalState; createdApp?: ApplicationParams };
