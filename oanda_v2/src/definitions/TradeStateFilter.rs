/// The state to filter the Trades by
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum TradeStateFilter {
    /// The Trades that are currently open
    Open,
    /// The Trades that have been fully closed
    Closed,
    /// The Trades that will be closed as soon as the trades’ instrument becomes tradeable
    CloseWhenTradeable,
    /// The Trades that are in any of the possible states listed above.
    All,
}
