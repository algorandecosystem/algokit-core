/**
 * Fields for a payment transaction.
 *
 * Definition:
 * data/transactions/payment.go : PaymentTxnFields
 */
export type TransactionPayment = { amount: bigint; "close-amount"?: number; "close-remainder-to"?: string; receiver: string };
