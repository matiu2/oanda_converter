use crate::definitions::transaction_id::TransactionID;
use crate::endpoints::transaction::Transaction;
/// The details of the requested Transaction are provided.
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
        Self {
            transaction: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
