use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct FinancingDayOfWeek {
    /// The day of the week to charge the financing.
    day_of_week: Option<DayOfWeek>,
    /// The number of days worth of financing to be charged on
    /// dayOfWeek.
    days_charged: Option<integer>,
}
