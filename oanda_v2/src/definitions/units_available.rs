use definitions::units_available_details::UnitsAvailableDetails;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct UnitsAvailable {
    /// The number of units that are available to be traded using
    /// an Order with a positionFill option of “DEFAULT”. For an
    /// Account with hedging enabled, this value will be the same
    /// as the “OPEN_ONLY” value. For an Account without hedging
    /// enabled, this value will be the same as the “REDUCE_FIRST”
    /// value.
    default: Option<UnitsAvailableDetails>,
    /// The number of units that may are available to be traded with
    /// an Order with a positionFill option of “REDUCE_FIRST”.
    reduce_first: Option<UnitsAvailableDetails>,
    /// The number of units that may are available to be traded with
    /// an Order with a positionFill option of “REDUCE_ONLY”.
    reduce_only: Option<UnitsAvailableDetails>,
    /// The number of units that may are available to be traded with
    /// an Order with a positionFill option of “OPEN_ONLY”.
    open_only: Option<UnitsAvailableDetails>,
}
impl Default for UnitsAvailable {
    fn default() -> Self {
        use Default::default;
        Self {
            default: default(),
            reduce_first: default(),
            reduce_only: default(),
            open_only: default(),
        }
    }
}
