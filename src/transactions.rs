use rust_decimal::Decimal;
use serde::Deserialize;

use crate::market::Currency;

/// Represents a transaction on an account.
#[derive(Debug, Deserialize)]
pub struct Transaction {
	pub row_index: u64,
	pub timestamp: u64,
	pub balance: Decimal,
	pub available: Decimal,
	pub balance_delta: Decimal,
	pub available_delta: Decimal,
	pub currency: Currency,
	pub description: String,
}

/// Contains a list of transactions.
#[derive(Debug, Deserialize)]
pub struct ListTransactionsResponse {
	pub id: String,
	pub transactions: Vec<Transaction>,
}

/// Contains a list of pending transactions.
#[derive(Debug, Deserialize)]
pub struct ListPendingTransactionsResponse {
	pub id: String,
	pub pending: Vec<Transaction>,
}
