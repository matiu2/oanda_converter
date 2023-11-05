use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct MarketOrderPositionCloseout {
    /// The instrument of the Position being closed out.
    instrument: Option<InstrumentName>,
    /// Indication of how much of the Position to close. Either
    /// “ALL”, or a DecimalNumber reflection a partial close of
    /// the Trade. The DecimalNumber must always be positive, and
    /// represent a number that doesn’t exceed the absolute size of
    /// the Position.
    units: Option<string>,
}
