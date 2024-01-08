use crate::definitions::decimal_number::DecimalNumber;
use crate::chrono::DateTime;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct LiquidityRegenerationScheduleStep {
    /// The timestamp of the schedule step.
    timestamp: Option<DateTime>,
    /// The amount of bid liquidity used at this step in the
    /// schedule.
    bid_liquidity_used: Option<DecimalNumber>,
    /// The amount of ask liquidity used at this step in the
    /// schedule.
    ask_liquidity_used: Option<DecimalNumber>,
}
impl Default for LiquidityRegenerationScheduleStep {
    fn default() -> Self {
        Self {
            timestamp: Default::default(),
            bid_liquidity_used: Default::default(),
            ask_liquidity_used: Default::default(),
        }
    }
}
