/// The reason that the Fixed Price Order was created
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum FixedPriceOrderReason {
    /// The Fixed Price Order was created as part of a platform
    /// account migration
    PlatformAccountMigration,
    /// The Fixed Price Order was created to close a Trade as part
    /// of division account migration
    TradeCloseDivisionAccountMigration,
    /// The Fixed Price Order was created to close a Trade
    /// administratively
    TradeCloseAdministrativeAction,
}
