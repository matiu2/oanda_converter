use definitions::account_id::AccountID;
use definitions::request_id::RequestID;
use definitions::transaction_type::TransactionType;
use definitions::transaction_id::TransactionID;
use chrono::DateTime;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct MarginCallEnterTransaction {
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
    /// The Type of the Transaction. Always set to
    /// “MARGIN_CALL_ENTER” for an MarginCallEnterTransaction.
    #[serde_inline_default("MARGIN_CALL_ENTER")]
    r#type: TransactionType,
}
impl Default for MarginCallEnterTransaction {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            time: default(),
            user_id: default(),
            account_id: default(),
            batch_id: default(),
            request_id: default(),
            r#type: "MARGIN_CALL_ENTER",
        }
    }
}
