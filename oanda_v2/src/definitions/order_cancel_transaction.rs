use definitions::transaction_id::TransactionID;
use definitions::transaction_type::TransactionType;
use definitions::request_id::RequestID;
use definitions::account_id::AccountID;
use definitions::order_id::OrderID;
use chrono::DateTime;
use definitions::order_cancel_reason::OrderCancelReason;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct OrderCancelTransaction {
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
    /// The Type of the Transaction. Always set to “ORDER_CANCEL”
    /// for an OrderCancelTransaction.
    #[serde(default = "ORDER_CANCEL")]
    r#type: TransactionType,
    /// The ID of the Order cancelled
    order_id: Option<OrderID>,
    /// The client ID of the Order cancelled (only provided if the
    /// Order has a client Order ID).
    client_order_id: Option<OrderID>,
    /// The reason that the Order was cancelled.
    reason: Option<OrderCancelReason>,
    /// The ID of the Order that replaced this Order (only provided
    /// if this Order was cancelled for replacement).
    replaced_by_order_id: Option<OrderID>,
}
