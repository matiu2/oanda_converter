use serde::{Serialize, Deserialize};
/// The financing mode of an Account
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum AccountFinancingMode {
    /// No financing is paid/charged for open Trades in the Account
    NoFinancing,
    /// Second-by-second financing is paid/charged for open Trades
    /// in the Account, both daily and when the the Trade is closed
    SecondBySecond,
    /// A full day’s worth of financing is paid/charged for open
    /// Trades in the Account daily at 5pm New York time
    Daily,
}
