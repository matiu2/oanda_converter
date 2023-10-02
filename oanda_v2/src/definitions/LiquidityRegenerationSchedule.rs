use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct LiquidityRegenerationSchedule {
    /// The steps in the Liquidity Regeneration Schedule
    steps: Vec<LiquidityRegenerationScheduleStep>,
}
