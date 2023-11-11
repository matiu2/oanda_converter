use serde::{Serialize, Deserialize};
/// The type of the Order.
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum CancellableOrderType {
    /// A Limit Order
    Limit,
    /// A Stop Order
    Stop,
    /// A Market-if-touched Order
    MarketIfTouched,
    /// A Take Profit Order
    TakeProfit,
    /// A Stop Loss Order
    StopLoss,
    /// A Guaranteed Stop Loss Order
    GuaranteedStopLoss,
    /// A Trailing Stop Loss Order
    TrailingStopLoss,
}
