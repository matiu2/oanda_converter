use crate::definitions::account_id::AccountID;
use chrono::Utc;
use chrono::DateTime;
use crate::definitions::transaction_id::TransactionID;
use crate::definitions::request_id::RequestID;
use serde_inline_default::serde_inline_default;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Transaction {
    /// The Transaction’s Identifier.
    id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    time: Option<DateTime<Utc>>,
    /// The ID of the user that initiated the creation of the
    /// Transaction.
    user_id: Option<integer>,
    /// The ID of the Account the Transaction was created for.
    account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to.
    /// Transactions in the same batch are applied to the Account
    /// simultaneously.
    batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the
    /// transaction.
    request_id: Option<RequestID>,
}
impl Default for Transaction {
    fn default() -> Self {
        Self {
            id: Default::default(),
            time: Default::default(),
            user_id: Default::default(),
            account_id: Default::default(),
            batch_id: Default::default(),
            request_id: Default::default(),
        }
    }
}
