use crate::definitions::position_side::PositionSide;
use crate::definitions::instrument_name::InstrumentName;
use serde_inline_default::serde_inline_default;
use crate::definitions::account_units::AccountUnits;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Position {
    /// The Position’s Instrument.
    instrument: Option<InstrumentName>,
    /// Profit/loss realized by the Position over the lifetime of
    /// the Account.
    pl: Option<AccountUnits>,
    /// The unrealized profit/loss of all open Trades that
    /// contribute to this Position.
    unrealized_pl: Option<AccountUnits>,
    /// Margin currently used by the Position.
    margin_used: Option<AccountUnits>,
    /// Profit/loss realized by the Position since the Account’s
    /// resettablePL was last reset by the client.
    resettable_pl: Option<AccountUnits>,
    /// The total amount of financing paid/collected for this
    /// instrument over the lifetime of the Account.
    financing: Option<AccountUnits>,
    /// The total amount of commission paid for this instrument over
    /// the lifetime of the Account.
    commission: Option<AccountUnits>,
    /// The total amount of dividend adjustment paid for this
    /// instrument over the lifetime of the Account.
    dividend_adjustment: Option<AccountUnits>,
    /// The total amount of fees charged over the lifetime of the
    /// Account for the execution of guaranteed Stop Loss Orders for
    /// this instrument.
    guaranteed_execution_fees: Option<AccountUnits>,
    /// The details of the long side of the Position.
    long: Option<PositionSide>,
    /// The details of the short side of the Position.
    short: Option<PositionSide>,
}
impl Default for Position {
    fn default() -> Self {
        Self {
            instrument: Default::default(),
            pl: Default::default(),
            unrealized_pl: Default::default(),
            margin_used: Default::default(),
            resettable_pl: Default::default(),
            financing: Default::default(),
            commission: Default::default(),
            dividend_adjustment: Default::default(),
            guaranteed_execution_fees: Default::default(),
            long: Default::default(),
            short: Default::default(),
        }
    }
}
