/// The reason that the Guaranteed Stop Loss Order was initiated
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum GuaranteedStopLossOrderReason {
    /// The Guaranteed Stop Loss Order was initiated at the request
    /// of a client
    ClientOrder,
    /// The Guaranteed Stop Loss Order was initiated as a
    /// replacement for an existing Order
    Replacement,
    /// The Guaranteed Stop Loss Order was initiated automatically
    /// when an Order was filled that opened a new Trade requiring a
    /// Guaranteed Stop Loss Order.
    OnFill,
}
