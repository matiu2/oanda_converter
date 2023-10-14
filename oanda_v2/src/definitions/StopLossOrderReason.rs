/// The reason that the Stop Loss Order was initiated
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum StopLossOrderReason {
    /// The Stop Loss Order was initiated at the request of a client
    ClientOrder,
    /// The Stop Loss Order was initiated as a replacement for an existing Order
    Replacement,
    /// The Stop Loss Order was initiated automatically when an Order was filled that opened a new Trade requiring a Stop Loss Order.
    OnFill,
}
