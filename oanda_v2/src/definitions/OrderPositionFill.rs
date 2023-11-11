use serde::{Serialize, Deserialize};
/// Specification of how Positions in the Account are modified
/// when the Order is filled.
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum OrderPositionFill {
    /// When the Order is filled, only allow Positions to be opened
    /// or extended.
    OpenOnly,
    /// When the Order is filled, always fully reduce an existing
    /// Position before opening a new Position.
    ReduceFirst,
    /// When the Order is filled, only reduce an existing Position.
    ReduceOnly,
    /// When the Order is filled, use REDUCE_FIRST behaviour for
    /// non-client hedging Accounts, and OPEN_ONLY behaviour for
    /// client hedging Accounts.
    Default,
}
