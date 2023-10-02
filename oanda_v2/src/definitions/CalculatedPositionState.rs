use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct CalculatedPositionState {
    /// The Position’s Instrument.
    instrument: Option<InstrumentName>,
    /// The Position’s net unrealized profit/loss
    netUnrealizedPL: Option<AccountUnits>,
    /// The unrealized profit/loss of the Position’s long open Trades
    longUnrealizedPL: Option<AccountUnits>,
    /// The unrealized profit/loss of the Position’s short open Trades
    shortUnrealizedPL: Option<AccountUnits>,
    /// Margin currently used by the Position.
    marginUsed: Option<AccountUnits>,
}
