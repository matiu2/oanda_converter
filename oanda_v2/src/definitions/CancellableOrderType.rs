/// The type of the Order.
pub enum CancellableOrderType {
    /// A Limit Order
    LIMIT,
    /// A Stop Order
    STOP,
    /// A Market-if-touched Order
    MARKET_IF_TOUCHED,
    /// A Take Profit Order
    TAKE_PROFIT,
    /// A Stop Loss Order
    STOP_LOSS,
    /// A Guaranteed Stop Loss Order
    GUARANTEED_STOP_LOSS,
    /// A Trailing Stop Loss Order
    TRAILING_STOP_LOSS,
}
