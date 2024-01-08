use endpoints::position::responses::close::Close404;
use definitions::transaction_id::TransactionID;
use definitions::order_cancel_transaction::OrderCancelTransaction;
use definitions::order_fill_transaction::OrderFillTransaction;
use definitions::market_order_transaction::MarketOrderTransaction;
use definitions::market_order_reject_transaction::MarketOrderRejectTransaction;
use endpoints::position::responses::close::Close400;
use serde::{Serialize, Deserialize};
/// The Position closeout request has been successfully
/// processed.
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Close200 {
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
impl Default for Close200 {
    fn default() -> Self {
        use Default::default;
        Self {
            long_order_create_transaction: default(),
            long_order_fill_transaction: default(),
            long_order_cancel_transaction: default(),
            short_order_create_transaction: default(),
            short_order_fill_transaction: default(),
            short_order_cancel_transaction: default(),
            related_transaction_i_ds: default(),
            last_transaction_id: default(),
        }
    }
}
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
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
        use Default::default;
        Self {
            long_order_reject_transaction: default(),
            short_order_reject_transaction: default(),
            related_transaction_i_ds: default(),
            last_transaction_id: default(),
            error_code: default(),
            error_message: default(),
        }
    }
}
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
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
        use Default::default;
        Self {
            long_order_reject_transaction: default(),
            short_order_reject_transaction: default(),
            related_transaction_i_ds: default(),
            last_transaction_id: default(),
            error_code: default(),
            error_message: default(),
        }
    }
}
#[derive(Debug)]
pub enum Error {
    E400(Close400),
    E404(Close404),
}
