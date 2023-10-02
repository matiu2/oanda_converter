/// The reason that the Fixed Price Order was created
pub enum FixedPriceOrderReason {
    /// The Fixed Price Order was created as part of a platform account migration
    PLATFORM_ACCOUNT_MIGRATION,
    /// The Fixed Price Order was created to close a Trade as part of division account migration
    TRADE_CLOSE_DIVISION_ACCOUNT_MIGRATION,
    /// The Fixed Price Order was created to close a Trade administratively
    TRADE_CLOSE_ADMINISTRATIVE_ACTION,
}
