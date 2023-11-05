/// For Accounts that support guaranteed Stop Loss Orders,
/// describes the actions that can be be performed on guaranteed
/// Stop Loss Orders.
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum GuaranteedStopLossOrderMutability {
    /// Once a guaranteed Stop Loss Order has been created it cannot
    /// be replaced or cancelled.
    Fixed,
    /// An existing guaranteed Stop Loss Order can only be replaced,
    /// not cancelled.
    Replaceable,
    /// Once a guaranteed Stop Loss Order has been created it can be
    /// either replaced or cancelled.
    Cancelable,
    /// An existing guaranteed Stop Loss Order can only be replaced
    /// to widen the gap from the current price, not cancelled.
    PriceWidenOnly,
}
