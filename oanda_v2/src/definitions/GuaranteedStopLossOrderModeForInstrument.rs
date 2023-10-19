/// The overall behaviour of the Account regarding Guaranteed Stop Loss Orders for a specific Instrument.
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum GuaranteedStopLossOrderModeForInstrument {
    /// The Account is not permitted to create Guaranteed Stop Loss Orders for this Instrument.
    Disabled,
    /// The Account is able, but not required to have Guaranteed Stop Loss Orders for open Trades for this Instrument.
    Allowed,
    /// The Account is required to have Guaranteed Stop Loss Orders for all open Trades for this Instrument.
    Required,
}
