use definitions::market_order_transaction::MarketOrderTransaction;
use endpoints::position::responses::close::Close400;
use definitions::order_fill_transaction::OrderFillTransaction;
use definitions::order_cancel_transaction::OrderCancelTransaction;
use definitions::transaction_id::TransactionID;
use definitions::market_order_reject_transaction::MarketOrderRejectTransaction;
use endpoints::position::responses::close::Close404;
use serde::{Serialize, Deserialize};
/// The Trade has been closed as requested
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Close200 {
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
impl Default for Close200 {
    fn default() -> Self {
        use Default::default;
        Self {
            order_create_transaction: default(),
            order_fill_transaction: default(),
            order_cancel_transaction: default(),
            related_transaction_i_ds: default(),
            last_transaction_id: default(),
        }
    }
}
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
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
        use Default::default;
        Self {
            order_reject_transaction: default(),
            error_code: default(),
            error_message: default(),
        }
    }
}
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
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
        use Default::default;
        Self {
            order_reject_transaction: default(),
            last_transaction_id: default(),
            related_transaction_i_ds: default(),
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
