/// The current state of the Trade.
pub enum TradeState {
    /// The Trade is currently open
    OPEN,
    /// The Trade has been fully closed
    CLOSED,
    /// The Trade will be closed as soon as the trade’s instrument becomes tradeable
    CLOSE_WHEN_TRADEABLE,
}
