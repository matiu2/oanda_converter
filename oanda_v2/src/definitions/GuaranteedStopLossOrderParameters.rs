use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct GuaranteedStopLossOrderParameters {
    /// The current guaranteed Stop Loss Order mutability setting of the Account when market is open.
    mutabilityMarketOpen: Option<GuaranteedStopLossOrderMutability>,
    /// The current guaranteed Stop Loss Order mutability setting of the Account when market is halted.
    mutabilityMarketHalted: Option<GuaranteedStopLossOrderMutability>,
}
