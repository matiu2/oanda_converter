use definitions::day_of_week::DayOfWeek;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct FinancingDayOfWeek {
    /// The day of the week to charge the financing.
    day_of_week: Option<DayOfWeek>,
    /// The number of days worth of financing to be charged on
    /// dayOfWeek.
    days_charged: Option<integer>,
}
impl Default for FinancingDayOfWeek {
    fn default() -> Self {
        use Default::default;
        Self {
            day_of_week: default(),
            days_charged: default(),
        }
    }
}
