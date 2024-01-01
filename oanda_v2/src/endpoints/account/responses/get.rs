use definitions::transaction_id::TransactionID;
use endpoints::account::Account;
use serde::{Serialize, Deserialize};
/// The full Account details are provided
use serde::{Serialize, Deserialize};
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
        use Default::default;
        Self {
            account: default(),
            last_transaction_id: default(),
        }
    }
}
