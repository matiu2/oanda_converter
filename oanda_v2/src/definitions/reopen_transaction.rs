use definitions::request_id::RequestID;
use chrono::DateTime;
use definitions::account_id::AccountID;
use definitions::transaction_id::TransactionID;
use definitions::transaction_type::TransactionType;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct ReopenTransaction {
    /// The Transaction’s Identifier.
    id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    time: Option<DateTime>,
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
    /// The Type of the Transaction. Always set to “REOPEN” in a
    /// ReopenTransaction.
    #[serde_inline_default("REOPEN")]
    r#type: TransactionType,
}
impl Default for ReopenTransaction {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            time: default(),
            user_id: default(),
            account_id: default(),
            batch_id: default(),
            request_id: default(),
            r#type: "REOPEN",
        }
    }
}
