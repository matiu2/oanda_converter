use crate::definitions::decimal_number::DecimalNumber;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct InstrumentCommission {
    /// The commission amount (in the Account’s home currency)
    /// charged per unitsTraded of the instrument
    commission: Option<DecimalNumber>,
    /// The number of units traded that the commission amount is
    /// based on.
    units_traded: Option<DecimalNumber>,
    /// The minimum commission amount (in the Account’s home
    /// currency) that is charged when an Order is filled for this
    /// instrument.
    minimum_commission: Option<DecimalNumber>,
}
impl Default for InstrumentCommission {
    fn default() -> Self {
        Self {
            commission: Default::default(),
            units_traded: Default::default(),
            minimum_commission: Default::default(),
        }
    }
}
