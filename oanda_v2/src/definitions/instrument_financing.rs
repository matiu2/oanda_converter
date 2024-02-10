use serde_inline_default::serde_inline_default;
use crate::definitions::decimal_number::DecimalNumber;
use crate::definitions::financing_day_of_week::FinancingDayOfWeek;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct InstrumentFinancing {
    /// The financing rate to be used for a long position for the
    /// instrument. The value is in decimal rather than percentage
    /// points, i.e. 5% is represented as 0.05.
    long_rate: Option<DecimalNumber>,
    /// The financing rate to be used for a short position for the
    /// instrument. The value is in decimal rather than percentage
    /// points, i.e. 5% is represented as 0.05.
    short_rate: Option<DecimalNumber>,
    /// The days of the week to debit or credit financing charges;
    /// the exact time of day at which to charge the financing is
    /// set in the DivisionTradingGroup for the client’s account.
    financing_days_of_week: Vec<FinancingDayOfWeek>,
}
impl Default for InstrumentFinancing {
    fn default() -> Self {
        Self {
            long_rate: Default::default(),
            short_rate: Default::default(),
            financing_days_of_week: Default::default(),
        }
    }
}
