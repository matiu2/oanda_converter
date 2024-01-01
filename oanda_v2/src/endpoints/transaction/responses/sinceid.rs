use definitions::transaction_id::TransactionID;
use endpoints::transaction::Transaction;
use serde::{Serialize, Deserialize};
/// The requested time range of Transactions are provided.
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Sinceid200 {
    /// The list of Transactions that satisfy the request.
    transactions: Vec<Transaction>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Sinceid200 {
    fn default() -> Self {
        use Default::default;
        Self {
            transactions: default(),
            last_transaction_id: default(),
        }
    }
}
