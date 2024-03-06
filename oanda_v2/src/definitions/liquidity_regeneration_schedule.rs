use serde_inline_default::serde_inline_default;
use crate::definitions::liquidity_regeneration_schedule_step::LiquidityRegenerationScheduleStep;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct LiquidityRegenerationSchedule {
    /// The steps in the Liquidity Regeneration Schedule
    steps: Vec<LiquidityRegenerationScheduleStep>,
}
impl Default for LiquidityRegenerationSchedule {
    fn default() -> Self {
        Self { steps: Default::default() }
    }
}
