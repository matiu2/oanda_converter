use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct UnitsAvailableDetails {
    /// The units available for long Orders.
    long: Option<DecimalNumber>,
    /// The units available for short Orders.
    short: Option<DecimalNumber>,
}
