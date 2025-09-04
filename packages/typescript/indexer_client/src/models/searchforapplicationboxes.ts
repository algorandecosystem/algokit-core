import type { BoxDescriptor } from "./index";

/**
 * Box names of an application
 */
export type SearchForApplicationBoxes = { "application-id": number; boxes: BoxDescriptor[]; "next-token"?: string };
