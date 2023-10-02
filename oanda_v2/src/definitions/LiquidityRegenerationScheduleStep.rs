use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct LiquidityRegenerationScheduleStep {
    /// The timestamp of the schedule step.
    timestamp: Option<DateTime>,
    /// The amount of bid liquidity used at this step in the schedule.
    bidLiquidityUsed: Option<DecimalNumber>,
    /// The amount of ask liquidity used at this step in the schedule.
    askLiquidityUsed: Option<DecimalNumber>,
}
