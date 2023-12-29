use serde::{Serialize, Deserialize};
/// The Account summary are provided
#[derive(Serialize, Deserialize)]
struct Summary200 {
    /// The summary of the requested Account.
    account: Option<AccountSummary>,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
}
