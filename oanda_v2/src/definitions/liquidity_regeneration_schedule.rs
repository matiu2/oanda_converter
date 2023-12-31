use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct LiquidityRegenerationSchedule {
    /// The steps in the Liquidity Regeneration Schedule
    steps: Vec<LiquidityRegenerationScheduleStep>,
}
