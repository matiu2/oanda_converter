use endpoints::transaction::Transaction;
use definitions::transaction_id::TransactionID;
use endpoints::order::responses::put::Put400;
use definitions::order_cancel_transaction::OrderCancelTransaction;
use definitions::order_fill_transaction::OrderFillTransaction;
use endpoints::order::responses::put::Put404;
use serde::{Serialize, Deserialize};
/// The Order was successfully cancelled and replaced
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Put201 {
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
impl Default for Put201 {
    fn default() -> Self {
        use Default::default;
        Self {
            order_cancel_transaction: default(),
            order_create_transaction: default(),
            order_fill_transaction: default(),
            order_reissue_transaction: default(),
            order_reissue_reject_transaction: default(),
            replacing_order_cancel_transaction: default(),
            related_transaction_i_ds: default(),
            last_transaction_id: default(),
        }
    }
}
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
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
        use Default::default;
        Self {
            order_reject_transaction: default(),
            related_transaction_i_ds: default(),
            last_transaction_id: default(),
            error_code: default(),
            error_message: default(),
        }
    }
}
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
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
        use Default::default;
        Self {
            order_cancel_reject_transaction: default(),
            related_transaction_i_ds: default(),
            last_transaction_id: default(),
            error_code: default(),
            error_message: default(),
        }
    }
}
#[derive(Debug)]
pub enum Error {
    E400(Put400),
    E404(Put404),
}
