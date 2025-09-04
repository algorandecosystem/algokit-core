/**
 * A health check response.
 */
export type HealthCheck = {
  version: string;
  data?: {};
  round: bigint;
  "is-migrating": boolean;
  "db-available": boolean;
  message: string;
  errors?: string[];
};
