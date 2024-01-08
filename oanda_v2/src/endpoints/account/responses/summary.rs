use crate::definitions::transaction_id::TransactionID;
use crate::definitions::account_summary::AccountSummary;
/// The Account summary are provided
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
        Self {
            account: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
