use serde::{Serialize, Deserialize};
/// The requested time range of Transactions are provided.
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Idrange200 {
    /// The list of Transactions that satisfy the request.
    transactions: Vec<Transaction>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
