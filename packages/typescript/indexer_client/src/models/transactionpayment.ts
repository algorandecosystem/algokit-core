/**
 * Fields for a payment transaction.
 *
 * Definition:
 * data/transactions/payment.go : PaymentTxnFields
 */
export type TransactionPayment = { amount: bigint; closeAmount?: bigint; closeRemainderTo?: string; receiver: string };
