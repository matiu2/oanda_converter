use crate::definitions::transaction_id::TransactionID;
use crate::endpoints::account::Account;
/// The full Account details are provided
#[derive(Serialize, Deserialize)]
pub struct Get200 {
    /// The full details of the requested Account.
    account: Option<Account>,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
}
impl Default for Get200 {
    fn default() -> Self {
        Self {
            account: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
