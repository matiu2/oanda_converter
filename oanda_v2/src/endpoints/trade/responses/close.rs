/// The Trade has been closed as requested
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Close {
    /// The MarketOrder Transaction created to close the Trade.
    order_create_transaction: Option<MarketOrderTransaction>,
    /// The OrderFill Transaction that fills the Trade-closing
    /// MarketOrder and closes the Trade.
    order_fill_transaction: Option<OrderFillTransaction>,
    /// The OrderCancel Transaction that immediately cancelled the
    /// Trade-closing MarketOrder.
    order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Close {
    fn default() -> Self {
        Self {
            order_create_transaction: Default::default(),
            order_fill_transaction: Default::default(),
            order_cancel_transaction: Default::default(),
            related_transaction_i_ds: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
use serde::{Serialize, Deserialize};
/// The Trade cannot be closed as requested.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Close400 {
    /// The MarketOrderReject Transaction that rejects the creation
    /// of the Trade- closing MarketOrder.
    order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<String>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: String,
}
impl Default for Close400 {
    fn default() -> Self {
        Self {
            order_reject_transaction: Default::default(),
            error_code: Default::default(),
            error_message: Default::default(),
        }
    }
}
/// The Account or Trade specified does not exist.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Close404 {
    /// The MarketOrderReject Transaction that rejects the creation
    /// of the Trade- closing MarketOrder. Only present if the
    /// Account exists.
    order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The ID of the most recent Transaction created for the
    /// Account. Only present if the Account exists.
    last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request. Only present if the Account exists.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<String>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: String,
}
impl Default for Close404 {
    fn default() -> Self {
        Self {
            order_reject_transaction: Default::default(),
            last_transaction_id: Default::default(),
            related_transaction_i_ds: Default::default(),
            error_code: Default::default(),
            error_message: Default::default(),
        }
    }
}
#[derive(Debug)]
pub enum Error {
    E400(Close400),
    E404(Close404),
}
