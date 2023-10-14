use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct Position {
    /// The Position’s Instrument.
    instrument: Option<InstrumentName>,
    /// Profit/loss realized by the Position over the lifetime of the Account.
    pl: Option<AccountUnits>,
    /// The unrealized profit/loss of all open Trades that contribute to this Position.
    unrealized_pl: Option<AccountUnits>,
    /// Margin currently used by the Position.
    margin_used: Option<AccountUnits>,
    /// Profit/loss realized by the Position since the Account’s resettablePL was last reset by the client.
    resettable_pl: Option<AccountUnits>,
    /// The total amount of financing paid/collected for this instrument over the lifetime of the Account.
    financing: Option<AccountUnits>,
    /// The total amount of commission paid for this instrument over the lifetime of the Account.
    commission: Option<AccountUnits>,
    /// The total amount of dividend adjustment paid for this instrument over the lifetime of the Account.
    dividend_adjustment: Option<AccountUnits>,
    /// The total amount of fees charged over the lifetime of the Account for the execution of guaranteed Stop Loss Orders for this instrument.
    guaranteed_execution_fees: Option<AccountUnits>,
    /// The details of the long side of the Position.
    long: Option<PositionSide>,
    /// The details of the short side of the Position.
    short: Option<PositionSide>,
}
