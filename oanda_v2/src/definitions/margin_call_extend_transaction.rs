use definitions::transaction_id::TransactionID;
use chrono::DateTime;
use definitions::account_id::AccountID;
use definitions::transaction_type::TransactionType;
use definitions::request_id::RequestID;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct MarginCallExtendTransaction {
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
    /// “MARGIN_CALL_EXTEND” for an MarginCallExtendTransaction.
    #[serde_inline_default("MARGIN_CALL_EXTEND")]
    r#type: TransactionType,
    /// The number of the extensions to the Account’s current margin
    /// call that have been applied. This value will be set to 1 for
    /// the first MarginCallExtend Transaction
    extension_number: Option<integer>,
}
impl Default for MarginCallExtendTransaction {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            time: default(),
            user_id: default(),
            account_id: default(),
            batch_id: default(),
            request_id: default(),
            r#type: "MARGIN_CALL_EXTEND",
            extension_number: default(),
        }
    }
}
