use crate::definitions::take_profit_order_reject_transaction::TakeProfitOrderRejectTransaction;
use crate::definitions::guaranteed_stop_loss_order_reject_transaction::GuaranteedStopLossOrderRejectTransaction;
use crate::definitions::guaranteed_stop_loss_order_transaction::GuaranteedStopLossOrderTransaction;
use crate::definitions::trailing_stop_loss_order_reject_transaction::TrailingStopLossOrderRejectTransaction;
use crate::definitions::transaction_id::TransactionID;
use crate::definitions::stop_loss_order_transaction::StopLossOrderTransaction;
use crate::definitions::order_cancel_reject_transaction::OrderCancelRejectTransaction;
use crate::definitions::trailing_stop_loss_order_transaction::TrailingStopLossOrderTransaction;
use crate::definitions::order_fill_transaction::OrderFillTransaction;
use crate::definitions::order_cancel_transaction::OrderCancelTransaction;
use crate::definitions::take_profit_order_transaction::TakeProfitOrderTransaction;
use crate::definitions::stop_loss_order_reject_transaction::StopLossOrderRejectTransaction;
/// The Trade’s dependent Orders have been modified as
/// requested.
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
impl Default for Orders200 {
    fn default() -> Self {
        Self {
            take_profit_order_cancel_transaction: Default::default(),
            take_profit_order_transaction: Default::default(),
            take_profit_order_fill_transaction: Default::default(),
            take_profit_order_created_cancel_transaction: Default::default(),
            stop_loss_order_cancel_transaction: Default::default(),
            stop_loss_order_transaction: Default::default(),
            stop_loss_order_fill_transaction: Default::default(),
            stop_loss_order_created_cancel_transaction: Default::default(),
            trailing_stop_loss_order_cancel_transaction: Default::default(),
            trailing_stop_loss_order_transaction: Default::default(),
            guaranteed_stop_loss_order_cancel_transaction: Default::default(),
            guaranteed_stop_loss_order_transaction: Default::default(),
            related_transaction_i_ds: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
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
impl Default for Orders400 {
    fn default() -> Self {
        Self {
            take_profit_order_cancel_reject_transaction: Default::default(),
            take_profit_order_reject_transaction: Default::default(),
            stop_loss_order_cancel_reject_transaction: Default::default(),
            stop_loss_order_reject_transaction: Default::default(),
            trailing_stop_loss_order_cancel_reject_transaction: Default::default(),
            trailing_stop_loss_order_reject_transaction: Default::default(),
            guaranteed_stop_loss_order_cancel_reject_transaction: Default::default(),
            guaranteed_stop_loss_order_reject_transaction: Default::default(),
            last_transaction_id: Default::default(),
            related_transaction_i_ds: Default::default(),
            error_code: Default::default(),
            error_message: Default::default(),
        }
    }
}
#[derive(Debug)]
pub enum Error {
    E400(Orders400),
}
