/// The reason that an Order was filled
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum OrderFillReason {
    /// The Order filled was a Limit Order
    LimitOrder,
    /// The Order filled was a Stop Order
    StopOrder,
    /// The Order filled was a Market-if-touched Order
    MarketIfTouchedOrder,
    /// The Order filled was a Take Profit Order
    TakeProfitOrder,
    /// The Order filled was a Stop Loss Order
    StopLossOrder,
    /// The Order filled was a Guaranteed Stop Loss Order
    GuaranteedStopLossOrder,
    /// The Order filled was a Trailing Stop Loss Order
    TrailingStopLossOrder,
    /// The Order filled was a Market Order
    MarketOrder,
    /// The Order filled was a Market Order used to explicitly close
    /// a Trade
    MarketOrderTradeClose,
    /// The Order filled was a Market Order used to explicitly close
    /// a Position
    MarketOrderPositionCloseout,
    /// The Order filled was a Market Order used for a Margin
    /// Closeout
    MarketOrderMarginCloseout,
    /// The Order filled was a Market Order used for a delayed Trade
    /// close
    MarketOrderDelayedTradeClose,
    /// The Order filled was a Fixed Price Order
    FixedPriceOrder,
    /// The Order filled was a Fixed Price Order created as part of
    /// a platform account migration
    FixedPriceOrderPlatformAccountMigration,
    /// The Order filled was a Fixed Price Order created to close a
    /// Trade as part of division account migration
    FixedPriceOrderDivisionAccountMigration,
    /// The Order filled was a Fixed Price Order created to close a
    /// Trade administratively
    FixedPriceOrderAdministrativeAction,
}
