/// The reason that the Stop Order was initiated
pub enum StopOrderReason {
    /// The Stop Order was initiated at the request of a client
    CLIENT_ORDER,
    /// The Stop Order was initiated as a replacement for an existing Order
    REPLACEMENT,
}
