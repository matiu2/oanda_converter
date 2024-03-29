/// The Order was cancelled as specified
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Cancel {
    /// The Transaction that cancelled the Order
    order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Cancel {
    fn default() -> Self {
        Self {
            order_cancel_transaction: Default::default(),
            related_transaction_i_ds: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
use serde::{Serialize, Deserialize};
/// The Account or Order specified does not exist.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Cancel404 {
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
    error_code: Option<String>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: String,
}
impl Default for Cancel404 {
    fn default() -> Self {
        Self {
            order_cancel_reject_transaction: Default::default(),
            related_transaction_i_ds: Default::default(),
            last_transaction_id: Default::default(),
            error_code: Default::default(),
            error_message: Default::default(),
        }
    }
}
#[derive(Debug)]
pub enum Error {
    E404(Cancel404),
}
