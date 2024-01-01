use definitions::account_summary::AccountSummary;
use definitions::transaction_id::TransactionID;
use serde::{Serialize, Deserialize};
/// The Account summary are provided
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Summary200 {
    /// The summary of the requested Account.
    account: Option<AccountSummary>,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
}
impl Default for Summary200 {
    fn default() -> Self {
        use Default::default;
        Self {
            account: default(),
            last_transaction_id: default(),
        }
    }
}
