use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct OrderCancelRejectTransaction {
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
    /// The Type of the Transaction. Always set to
    /// “ORDER_CANCEL_REJECT” for an OrderCancelRejectTransaction.
    #[serde_inline_default("ORDER_CANCEL_REJECT")]
    r#type: TransactionType,
    /// The ID of the Order intended to be cancelled
    order_id: Option<OrderID>,
    /// The client ID of the Order intended to be cancelled (only
    /// provided if the Order has a client Order ID).
    client_order_id: Option<OrderID>,
    /// The reason that the Reject Transaction was created
    reject_reason: Option<TransactionRejectReason>,
}
impl Default for OrderCancelRejectTransaction {
    fn default() -> Self {
        Self {
            id: Default::default(),
            time: Default::default(),
            user_id: Default::default(),
            account_id: Default::default(),
            batch_id: Default::default(),
            request_id: Default::default(),
            r#type: "ORDER_CANCEL_REJECT",
            order_id: Default::default(),
            client_order_id: Default::default(),
            reject_reason: Default::default(),
        }
    }
}
