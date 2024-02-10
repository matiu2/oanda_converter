use crate::definitions::day_of_week::DayOfWeek;
use serde_inline_default::serde_inline_default;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct FinancingDayOfWeek {
    /// The day of the week to charge the financing.
    day_of_week: Option<DayOfWeek>,
    /// The number of days worth of financing to be charged on
    /// dayOfWeek.
    days_charged: Option<integer>,
}
impl Default for FinancingDayOfWeek {
    fn default() -> Self {
        Self {
            day_of_week: Default::default(),
            days_charged: Default::default(),
        }
    }
}
