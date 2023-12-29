use serde::{Serialize, Deserialize};
/// The Order was cancelled as specified
#[derive(Serialize, Deserialize)]
struct Cancel200 {
    /// The Transaction that cancelled the Order
    order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
#[derive(Serialize, Deserialize)]
struct Cancel404 {
    /// The Transaction that rejected the cancellation of the Order.
    /// Only present if the Account exists.
    order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request. Only present if the Account exists.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account. Only present if the Account exists.
    last_transaction_id: Option<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<string>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: string,
}
#[derive(Debug)]
pub enum Error {
    E404(Cancel404),
}
