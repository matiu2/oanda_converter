/// The possible types of a Transaction
pub enum TransactionType {
    /// Account Create Transaction
    CREATE,
    /// Account Close Transaction
    CLOSE,
    /// Account Reopen Transaction
    REOPEN,
    /// Client Configuration Transaction
    CLIENT_CONFIGURE,
    /// Client Configuration Reject Transaction
    CLIENT_CONFIGURE_REJECT,
    /// Transfer Funds Transaction
    TRANSFER_FUNDS,
    /// Transfer Funds Reject Transaction
    TRANSFER_FUNDS_REJECT,
    /// Market Order Transaction
    MARKET_ORDER,
    /// Market Order Reject Transaction
    MARKET_ORDER_REJECT,
    /// Fixed Price Order Transaction
    FIXED_PRICE_ORDER,
    /// Limit Order Transaction
    LIMIT_ORDER,
    /// Limit Order Reject Transaction
    LIMIT_ORDER_REJECT,
    /// Stop Order Transaction
    STOP_ORDER,
    /// Stop Order Reject Transaction
    STOP_ORDER_REJECT,
    /// Market if Touched Order Transaction
    MARKET_IF_TOUCHED_ORDER,
    /// Market if Touched Order Reject Transaction
    MARKET_IF_TOUCHED_ORDER_REJECT,
    /// Take Profit Order Transaction
    TAKE_PROFIT_ORDER,
    /// Take Profit Order Reject Transaction
    TAKE_PROFIT_ORDER_REJECT,
    /// Stop Loss Order Transaction
    STOP_LOSS_ORDER,
    /// Stop Loss Order Reject Transaction
    STOP_LOSS_ORDER_REJECT,
    /// Guaranteed Stop Loss Order Transaction
    GUARANTEED_STOP_LOSS_ORDER,
    /// Guaranteed Stop Loss Order Reject Transaction
    GUARANTEED_STOP_LOSS_ORDER_REJECT,
    /// Trailing Stop Loss Order Transaction
    TRAILING_STOP_LOSS_ORDER,
    /// Trailing Stop Loss Order Reject Transaction
    TRAILING_STOP_LOSS_ORDER_REJECT,
    /// Order Fill Transaction
    ORDER_FILL,
    /// Order Cancel Transaction
    ORDER_CANCEL,
    /// Order Cancel Reject Transaction
    ORDER_CANCEL_REJECT,
    /// Order Client Extensions Modify Transaction
    ORDER_CLIENT_EXTENSIONS_MODIFY,
    /// Order Client Extensions Modify Reject Transaction
    ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT,
    /// Trade Client Extensions Modify Transaction
    TRADE_CLIENT_EXTENSIONS_MODIFY,
    /// Trade Client Extensions Modify Reject Transaction
    TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT,
    /// Margin Call Enter Transaction
    MARGIN_CALL_ENTER,
    /// Margin Call Extend Transaction
    MARGIN_CALL_EXTEND,
    /// Margin Call Exit Transaction
    MARGIN_CALL_EXIT,
    /// Delayed Trade Closure Transaction
    DELAYED_TRADE_CLOSURE,
    /// Daily Financing Transaction
    DAILY_FINANCING,
    /// Dividend Adjustment Transaction
    DIVIDEND_ADJUSTMENT,
    /// Reset Resettable PL Transaction
    RESET_RESETTABLE_PL,
}
