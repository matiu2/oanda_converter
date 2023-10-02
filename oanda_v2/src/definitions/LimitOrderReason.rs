/// The reason that the Limit Order was initiated
pub enum LimitOrderReason {
    /// The Limit Order was initiated at the request of a client
    CLIENT_ORDER,
    /// The Limit Order was initiated as a replacement for an existing Order
    REPLACEMENT,
}
