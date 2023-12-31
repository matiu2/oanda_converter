use endpoints::account::Account;
use definitions::transaction_id::TransactionID;
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
