use serde::{Serialize, Deserialize};
/// The reason that the Stop Order was initiated
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum StopOrderReason {
    /// The Stop Order was initiated at the request of a client
    ClientOrder,
    /// The Stop Order was initiated as a replacement for an
    /// existing Order
    Replacement,
}
