use serde::{Serialize, Deserialize};
/// The full Account details are provided
#[derive(Serialize, Deserialize)]
struct Get200 {
    /// The full details of the requested Account.
    account: Option<Account>,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
}
