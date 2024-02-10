use crate::definitions::decimal_number::DecimalNumber;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct UnitsAvailableDetails {
    /// The units available for long Orders.
    long: Option<DecimalNumber>,
    /// The units available for short Orders.
    short: Option<DecimalNumber>,
}
impl Default for UnitsAvailableDetails {
    fn default() -> Self {
        Self {
            long: Default::default(),
            short: Default::default(),
        }
    }
}
