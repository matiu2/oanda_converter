use serde::{Serialize, Deserialize};
/// The reason that the Trailing Stop Loss Order was initiated
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrailingStopLossOrderReason {
    /// The Trailing Stop Loss Order was initiated at the request of
    /// a client
    ClientOrder,
    /// The Trailing Stop Loss Order was initiated as a replacement
    /// for an existing Order
    Replacement,
    /// The Trailing Stop Loss Order was initiated automatically
    /// when an Order was filled that opened a new Trade requiring a
    /// Trailing Stop Loss Order.
    OnFill,
}
