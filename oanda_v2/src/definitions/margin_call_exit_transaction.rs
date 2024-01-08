use crate::definitions::request_id::RequestID;
use chrono::DateTime;
use crate::definitions::account_id::AccountID;
use crate::definitions::transaction_id::TransactionID;
use crate::definitions::transaction_type::TransactionType;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct MarginCallExitTransaction {
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
    /// “MARGIN_CALL_EXIT” for an MarginCallExitTransaction.
    #[serde_inline_default("MARGIN_CALL_EXIT")]
    r#type: TransactionType,
}
impl Default for MarginCallExitTransaction {
    fn default() -> Self {
        Self {
            id: Default::default(),
            time: Default::default(),
            user_id: Default::default(),
            account_id: Default::default(),
            batch_id: Default::default(),
            request_id: Default::default(),
            r#type: "MARGIN_CALL_EXIT",
        }
    }
}
