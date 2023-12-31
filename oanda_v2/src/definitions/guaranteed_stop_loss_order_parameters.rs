use definitions::guaranteed_stop_loss_order_mutability::GuaranteedStopLossOrderMutability;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct GuaranteedStopLossOrderParameters {
    /// The current guaranteed Stop Loss Order mutability setting of
    /// the Account when market is open.
    mutability_market_open: Option<GuaranteedStopLossOrderMutability>,
    /// The current guaranteed Stop Loss Order mutability setting of
    /// the Account when market is halted.
    mutability_market_halted: Option<GuaranteedStopLossOrderMutability>,
}
