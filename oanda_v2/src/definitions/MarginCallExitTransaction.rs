use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct MarginCallExitTransaction {
    /// The Transaction’s Identifier.
    id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    time: Option<DateTime>,
    /// The ID of the user that initiated the creation of the Transaction.
    user_id: Option<integer>,
    /// The ID of the Account the Transaction was created for.
    account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the Account simultaneously.
    batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “MARGIN_CALL_EXIT” for an MarginCallExitTransaction.
    #[serde(default = "MARGIN_CALL_EXIT")]
    r#type: TransactionType,
}
