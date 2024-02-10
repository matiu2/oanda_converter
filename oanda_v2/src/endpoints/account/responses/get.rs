use crate::endpoints::account::Account;
use crate::definitions::transaction_id::TransactionID;
/// The full Account details are provided
#[derive(Serialize, Deserialize)]
pub struct Get {
    /// The full details of the requested Account.
    account: Option<Account>,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
}
impl Default for Get {
    fn default() -> Self {
        Self {
            account: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
