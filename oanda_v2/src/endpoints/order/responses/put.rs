/// The Order was successfully cancelled and replaced
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Put {
    /// The Transaction that cancelled the Order to be replaced.
    order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction that created the replacing Order as
    /// requested.
    order_create_transaction: Option<Transaction>,
    /// The Transaction that filled the replacing Order. This is
    /// only provided when the replacing Order was immediately
    /// filled.
    order_fill_transaction: Option<OrderFillTransaction>,
    /// The Transaction that reissues the replacing Order. Only
    /// provided when the replacing Order was partially filled
    /// immediately and is configured to be reissued for its
    /// remaining units.
    order_reissue_transaction: Option<Transaction>,
    /// The Transaction that rejects the reissue of the Order.
    /// Only provided when the replacing Order was partially filled
    /// immediately and was configured to be reissued, however the
    /// reissue was rejected.
    order_reissue_reject_transaction: Option<Transaction>,
    /// The Transaction that cancelled the replacing Order. Only
    /// provided when the replacing Order was immediately cancelled.
    replacing_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Put {
    fn default() -> Self {
        Self {
            order_cancel_transaction: Default::default(),
            order_create_transaction: Default::default(),
            order_fill_transaction: Default::default(),
            order_reissue_transaction: Default::default(),
            order_reissue_reject_transaction: Default::default(),
            replacing_order_cancel_transaction: Default::default(),
            related_transaction_i_ds: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
use serde::{Serialize, Deserialize};
/// The Order specification was invalid
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Put400 {
    /// The Transaction that rejected the creation of the replacing
    /// Order
    order_reject_transaction: Option<Transaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<String>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: String,
}
impl Default for Put400 {
    fn default() -> Self {
        Self {
            order_reject_transaction: Default::default(),
            related_transaction_i_ds: Default::default(),
            last_transaction_id: Default::default(),
            error_code: Default::default(),
            error_message: Default::default(),
        }
    }
}
/// The Account or Order specified does not exist.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Put404 {
    /// The Transaction that rejected the cancellation of the Order
    /// to be replaced. Only present if the Account exists.
    order_cancel_reject_transaction: Option<Transaction>,
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
impl Default for Put404 {
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
    E400(Put400),
    E404(Put404),
}
