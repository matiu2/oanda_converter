use endpoints::transaction::Transaction;
use definitions::transaction_id::TransactionID;
use serde::{Serialize, Deserialize};
/// The details of the requested Transaction are provided.
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Get200 {
    /// The details of the Transaction requested
    transaction: Option<Transaction>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Get200 {
    fn default() -> Self {
        use Default::default;
        Self {
            transaction: default(),
            last_transaction_id: default(),
        }
    }
}
