use crate::definitions::account_units::AccountUnits;
use crate::definitions::instrument_name::InstrumentName;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct CalculatedPositionState {
    /// The Position’s Instrument.
    instrument: Option<InstrumentName>,
    /// The Position’s net unrealized profit/loss
    net_unrealized_pl: Option<AccountUnits>,
    /// The unrealized profit/loss of the Position’s long open
    /// Trades
    long_unrealized_pl: Option<AccountUnits>,
    /// The unrealized profit/loss of the Position’s short open
    /// Trades
    short_unrealized_pl: Option<AccountUnits>,
    /// Margin currently used by the Position.
    margin_used: Option<AccountUnits>,
}
impl Default for CalculatedPositionState {
    fn default() -> Self {
        Self {
            instrument: Default::default(),
            net_unrealized_pl: Default::default(),
            long_unrealized_pl: Default::default(),
            short_unrealized_pl: Default::default(),
            margin_used: Default::default(),
        }
    }
}
