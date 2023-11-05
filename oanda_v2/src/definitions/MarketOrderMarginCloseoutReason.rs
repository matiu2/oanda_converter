/// The reason that the Market Order was created to perform a
/// margin closeout
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum MarketOrderMarginCloseoutReason {
    /// Trade closures resulted from violating OANDA’s margin policy
    MarginCheckViolation,
    /// Trade closures came from a margin closeout event resulting
    /// from regulatory conditions placed on the Account’s margin
    /// call
    RegulatoryMarginCallViolation,
    /// Trade closures resulted from violating the margin policy
    /// imposed by regulatory requirements
    RegulatoryMarginCheckViolation,
}
