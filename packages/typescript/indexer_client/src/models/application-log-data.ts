/**
 * Stores the global information associated with an application.
 */
export type ApplicationLogData = {
  /**
   * Transaction ID
   */
  txid: string;

  /**
   * Logs for the application being executed by the transaction.
   */
  logs: string[];
};
