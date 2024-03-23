/// The Position closeout request has been successfully
/// processed.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Close {
    /// The MarketOrderTransaction created to close the long
    /// Position.
    long_order_create_transaction: Option<MarketOrderTransaction>,
    /// OrderFill Transaction that closes the long Position
    long_order_fill_transaction: Option<OrderFillTransaction>,
    /// OrderCancel Transaction that cancels the MarketOrder created
    /// to close the long Position
    long_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The MarketOrderTransaction created to close the short
    /// Position.
    short_order_create_transaction: Option<MarketOrderTransaction>,
    /// OrderFill Transaction that closes the short Position
    short_order_fill_transaction: Option<OrderFillTransaction>,
    /// OrderCancel Transaction that cancels the MarketOrder created
    /// to close the short Position
    short_order_cancel_transaction: Option<OrderCancelTransaction>,
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
            long_order_create_transaction: Default::default(),
            long_order_fill_transaction: Default::default(),
            long_order_cancel_transaction: Default::default(),
            short_order_create_transaction: Default::default(),
            short_order_fill_transaction: Default::default(),
            short_order_cancel_transaction: Default::default(),
            related_transaction_i_ds: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
use serde::{Serialize, Deserialize};
/// The Parameters provided that describe the Position closeout
/// are invalid.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Close400 {
    /// The Transaction created that rejects the creation of a
    /// MarketOrder to close the long Position.
    long_order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The Transaction created that rejects the creation of a
    /// MarketOrder to close the short Position.
    short_order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
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
            long_order_reject_transaction: Default::default(),
            short_order_reject_transaction: Default::default(),
            related_transaction_i_ds: Default::default(),
            last_transaction_id: Default::default(),
            error_code: Default::default(),
            error_message: Default::default(),
        }
    }
}
/// The Account or one or more of the Positions specified does
/// not exist.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Close404 {
    /// The Transaction created that rejects the creation of a
    /// MarketOrder to close the long Position. Only present if the
    /// Account exists and a long Position was specified.
    long_order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The Transaction created that rejects the creation of a
    /// MarketOrder to close the short Position. Only present if the
    /// Account exists and a short Position was specified.
    short_order_reject_transaction: Option<MarketOrderRejectTransaction>,
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
impl Default for Close404 {
    fn default() -> Self {
        Self {
            long_order_reject_transaction: Default::default(),
            short_order_reject_transaction: Default::default(),
            related_transaction_i_ds: Default::default(),
            last_transaction_id: Default::default(),
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
