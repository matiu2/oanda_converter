/// For Accounts that support guaranteed Stop Loss Orders, describes the actions that can be be performed on guaranteed Stop Loss Orders.
pub enum GuaranteedStopLossOrderMutability {
    /// Once a guaranteed Stop Loss Order has been created it cannot be replaced or cancelled.
    FIXED,
    /// An existing guaranteed Stop Loss Order can only be replaced, not cancelled.
    REPLACEABLE,
    /// Once a guaranteed Stop Loss Order has been created it can be either replaced or cancelled.
    CANCELABLE,
    /// An existing guaranteed Stop Loss Order can only be replaced to widen the gap from the current price, not cancelled.
    PRICE_WIDEN_ONLY,
}
