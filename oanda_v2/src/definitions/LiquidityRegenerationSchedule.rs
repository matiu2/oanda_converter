use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct LiquidityRegenerationSchedule {
    /// The steps in the Liquidity Regeneration Schedule
    steps: Vec<LiquidityRegenerationScheduleStep>,
}
