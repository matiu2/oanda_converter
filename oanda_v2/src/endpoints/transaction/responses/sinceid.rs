use crate::definitions::transaction_id::TransactionID;
use crate::endpoints::transaction::Transaction;
use serde_inline_default::serde_inline_default;
/// The requested time range of Transactions are provided.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Sinceid {
    /// The list of Transactions that satisfy the request.
    transactions: Vec<Transaction>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Sinceid {
    fn default() -> Self {
        Self {
            transactions: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
