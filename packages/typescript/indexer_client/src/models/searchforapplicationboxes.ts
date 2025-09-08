import type { BoxDescriptor } from "./index";

/**
 * Box names of an application
 */
export type SearchForApplicationBoxes = { applicationId: bigint; boxes: BoxDescriptor[]; nextToken?: string };
