/// The current state of the Trade.
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum TradeState {
    /// The Trade is currently open
    Open,
    /// The Trade has been fully closed
    Closed,
    /// The Trade will be closed as soon as the trade’s instrument
    /// becomes tradeable
    CloseWhenTradeable,
}
