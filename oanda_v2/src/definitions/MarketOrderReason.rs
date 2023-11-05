/// The reason that the Market Order was created
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum MarketOrderReason {
    /// The Market Order was created at the request of a client
    ClientOrder,
    /// The Market Order was created to close a Trade at the request
    /// of a client
    TradeClose,
    /// The Market Order was created to close a Position at the
    /// request of a client
    PositionCloseout,
    /// The Market Order was created as part of a Margin Closeout
    MarginCloseout,
    /// The Market Order was created to close a trade marked for
    /// delayed closure
    DelayedTradeClose,
}
