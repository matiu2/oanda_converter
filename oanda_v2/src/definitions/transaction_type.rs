use serde::{Serialize, Deserialize};
/// The possible types of a Transaction
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    /// Account Create Transaction
    Create,
    /// Account Close Transaction
    Close,
    /// Account Reopen Transaction
    Reopen,
    /// Client Configuration Transaction
    ClientConfigure,
    /// Client Configuration Reject Transaction
    ClientConfigureReject,
    /// Transfer Funds Transaction
    TransferFunds,
    /// Transfer Funds Reject Transaction
    TransferFundsReject,
    /// Market Order Transaction
    MarketOrder,
    /// Market Order Reject Transaction
    MarketOrderReject,
    /// Fixed Price Order Transaction
    FixedPriceOrder,
    /// Limit Order Transaction
    LimitOrder,
    /// Limit Order Reject Transaction
    LimitOrderReject,
    /// Stop Order Transaction
    StopOrder,
    /// Stop Order Reject Transaction
    StopOrderReject,
    /// Market if Touched Order Transaction
    MarketIfTouchedOrder,
    /// Market if Touched Order Reject Transaction
    MarketIfTouchedOrderReject,
    /// Take Profit Order Transaction
    TakeProfitOrder,
    /// Take Profit Order Reject Transaction
    TakeProfitOrderReject,
    /// Stop Loss Order Transaction
    StopLossOrder,
    /// Stop Loss Order Reject Transaction
    StopLossOrderReject,
    /// Guaranteed Stop Loss Order Transaction
    GuaranteedStopLossOrder,
    /// Guaranteed Stop Loss Order Reject Transaction
    GuaranteedStopLossOrderReject,
    /// Trailing Stop Loss Order Transaction
    TrailingStopLossOrder,
    /// Trailing Stop Loss Order Reject Transaction
    TrailingStopLossOrderReject,
    /// Order Fill Transaction
    OrderFill,
    /// Order Cancel Transaction
    OrderCancel,
    /// Order Cancel Reject Transaction
    OrderCancelReject,
    /// Order Client Extensions Modify Transaction
    OrderClientExtensionsModify,
    /// Order Client Extensions Modify Reject Transaction
    OrderClientExtensionsModifyReject,
    /// Trade Client Extensions Modify Transaction
    TradeClientExtensionsModify,
    /// Trade Client Extensions Modify Reject Transaction
    TradeClientExtensionsModifyReject,
    /// Margin Call Enter Transaction
    MarginCallEnter,
    /// Margin Call Extend Transaction
    MarginCallExtend,
    /// Margin Call Exit Transaction
    MarginCallExit,
    /// Delayed Trade Closure Transaction
    DelayedTradeClosure,
    /// Daily Financing Transaction
    DailyFinancing,
    /// Dividend Adjustment Transaction
    DividendAdjustment,
    /// Reset Resettable PL Transaction
    ResetResettablePl,
}
