use serde_inline_default::serde_inline_default;
use crate::endpoints::transaction::Transaction;
use crate::definitions::transaction_id::TransactionID;
/// The details of the requested Transaction are provided.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Get {
    /// The details of the Transaction requested
    transaction: Option<Transaction>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Get {
    fn default() -> Self {
        Self {
            transaction: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
