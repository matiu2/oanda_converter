/// The financing mode of an Account
pub enum AccountFinancingMode {
    /// No financing is paid/charged for open Trades in the Account
    NO_FINANCING,
    /// Second-by-second financing is paid/charged for open Trades in the Account, both daily and when the the Trade is closed
    SECOND_BY_SECOND,
    /// A full day’s worth of financing is paid/charged for open Trades in the Account daily at 5pm New York time
    DAILY,
}
