/// The reason that the Guaranteed Stop Loss Order was initiated
pub enum GuaranteedStopLossOrderReason {
    /// The Guaranteed Stop Loss Order was initiated at the request of a client
    CLIENT_ORDER,
    /// The Guaranteed Stop Loss Order was initiated as a replacement for an existing Order
    REPLACEMENT,
    /// The Guaranteed Stop Loss Order was initiated automatically when an Order was filled that opened a new Trade requiring a Guaranteed Stop Loss Order.
    ON_FILL,
}
