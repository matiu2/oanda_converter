use serde::{Serialize, Deserialize};
/// The reason that an Account is being funded.
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum FundingReason {
    /// The client has initiated a funds transfer
    ClientFunding,
    /// Funds are being transferred between two Accounts.
    AccountTransfer,
    /// Funds are being transferred as part of a Division migration
    DivisionMigration,
    /// Funds are being transferred as part of a Site migration
    SiteMigration,
    /// Funds are being transferred as part of an Account adjustment
    Adjustment,
}
