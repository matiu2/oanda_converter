use serde::{Serialize, Deserialize};
/// The reason that the Take Profit Order was initiated
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TakeProfitOrderReason {
    /// The Take Profit Order was initiated at the request of
    /// a client
    ClientOrder,
    /// The Take Profit Order was initiated as a replacement for an
    /// existing Order
    Replacement,
    /// The Take Profit Order was initiated automatically when an
    /// Order was filled that opened a new Trade requiring a Take
    /// Profit Order.
    OnFill,
}
