use crate::definitions::transaction_id::TransactionID;
use crate::endpoints::transaction::Transaction;
/// The requested time range of Transactions are provided.
#[derive(Serialize, Deserialize)]
pub struct Idrange200 {
    /// The list of Transactions that satisfy the request.
    transactions: Vec<Transaction>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Idrange200 {
    fn default() -> Self {
        Self {
            transactions: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
