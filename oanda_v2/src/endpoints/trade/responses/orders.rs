use definitions::stop_loss_order_transaction::StopLossOrderTransaction;
use definitions::transaction_id::TransactionID;
use definitions::guaranteed_stop_loss_order_reject_transaction::GuaranteedStopLossOrderRejectTransaction;
use definitions::take_profit_order_reject_transaction::TakeProfitOrderRejectTransaction;
use definitions::guaranteed_stop_loss_order_transaction::GuaranteedStopLossOrderTransaction;
use endpoints::trade::responses::orders::Orders400;
use definitions::trailing_stop_loss_order_reject_transaction::TrailingStopLossOrderRejectTransaction;
use definitions::order_cancel_transaction::OrderCancelTransaction;
use definitions::trailing_stop_loss_order_transaction::TrailingStopLossOrderTransaction;
use definitions::order_cancel_reject_transaction::OrderCancelRejectTransaction;
use definitions::order_fill_transaction::OrderFillTransaction;
use definitions::stop_loss_order_reject_transaction::StopLossOrderRejectTransaction;
use definitions::take_profit_order_transaction::TakeProfitOrderTransaction;
use serde::{Serialize, Deserialize};
/// The Trade’s dependent Orders have been modified as
/// requested.
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Orders200 {
    /// The Transaction created that cancels the Trade’s existing
    /// Take Profit Order.
    take_profit_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that creates a new Take Profit Order
    /// for the Trade.
    take_profit_order_transaction: Option<TakeProfitOrderTransaction>,
    /// The Transaction created that immediately fills the Trade’s
    /// new Take Profit Order. Only provided if the new Take Profit
    /// Order was immediately filled.
    take_profit_order_fill_transaction: Option<OrderFillTransaction>,
    /// The Transaction created that immediately cancels the Trade’s
    /// new Take Profit Order. Only provided if the new Take Profit
    /// Order was immediately cancelled.
    take_profit_order_created_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that cancels the Trade’s existing
    /// Stop Loss Order.
    stop_loss_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that creates a new Stop Loss Order
    /// for the Trade.
    stop_loss_order_transaction: Option<StopLossOrderTransaction>,
    /// The Transaction created that immediately fills the Trade’s
    /// new Stop Order. Only provided if the new Stop Loss Order was
    /// immediately filled.
    stop_loss_order_fill_transaction: Option<OrderFillTransaction>,
    /// The Transaction created that immediately cancels the Trade’s
    /// new Stop Loss Order. Only provided if the new Stop Loss
    /// Order was immediately cancelled.
    stop_loss_order_created_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that cancels the Trade’s existing
    /// Trailing Stop Loss Order.
    trailing_stop_loss_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that creates a new Trailing Stop
    /// Loss Order for the Trade.
    trailing_stop_loss_order_transaction: Option<TrailingStopLossOrderTransaction>,
    /// The Transaction created that cancels the Trade’s existing
    /// Guaranteed Stop Loss Order.
    guaranteed_stop_loss_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The Transaction created that creates a new Guaranteed Stop
    /// Loss Order for the Trade.
    guaranteed_stop_loss_order_transaction: Option<GuaranteedStopLossOrderTransaction>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Orders400 {
    /// An OrderCancelRejectTransaction represents the rejection of
    /// the cancellation of an Order in the client’s Account.
    take_profit_order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
    /// A TakeProfitOrderRejectTransaction represents the rejection
    /// of the creation of a TakeProfit Order.
    take_profit_order_reject_transaction: Option<TakeProfitOrderRejectTransaction>,
    /// An OrderCancelRejectTransaction represents the rejection of
    /// the cancellation of an Order in the client’s Account.
    stop_loss_order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
    /// A StopLossOrderRejectTransaction represents the rejection of
    /// the creation of a StopLoss Order.
    stop_loss_order_reject_transaction: Option<StopLossOrderRejectTransaction>,
    /// An OrderCancelRejectTransaction represents the rejection of
    /// the cancellation of an Order in the client’s Account.
    trailing_stop_loss_order_cancel_reject_transaction: Option<
        OrderCancelRejectTransaction,
    >,
    /// A TrailingStopLossOrderRejectTransaction represents the
    /// rejection of the creation of a TrailingStopLoss Order.
    trailing_stop_loss_order_reject_transaction: Option<
        TrailingStopLossOrderRejectTransaction,
    >,
    /// An OrderCancelRejectTransaction represents the rejection of
    /// the cancellation of an Order in the client’s Account.
    guaranteed_stop_loss_order_cancel_reject_transaction: Option<
        OrderCancelRejectTransaction,
    >,
    /// A GuaranteedStopLossOrderRejectTransaction represents the
    /// rejection of the creation of a GuaranteedStopLoss Order.
    guaranteed_stop_loss_order_reject_transaction: Option<
        GuaranteedStopLossOrderRejectTransaction,
    >,
    /// The ID of the most recent Transaction created for the
    /// Account.
    last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<String>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: String,
}
#[derive(Debug)]
pub enum Error {
    E400(Orders400),
}
