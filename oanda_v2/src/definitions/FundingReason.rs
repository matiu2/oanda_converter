/// The reason that an Account is being funded.
pub enum FundingReason {
    /// The client has initiated a funds transfer
    CLIENT_FUNDING,
    /// Funds are being transferred between two Accounts.
    ACCOUNT_TRANSFER,
    /// Funds are being transferred as part of a Division migration
    DIVISION_MIGRATION,
    /// Funds are being transferred as part of a Site migration
    SITE_MIGRATION,
    /// Funds are being transferred as part of an Account adjustment
    ADJUSTMENT,
}
