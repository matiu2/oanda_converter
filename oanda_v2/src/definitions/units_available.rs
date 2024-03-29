use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
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
        Self {
            default: Default::default(),
            reduce_first: Default::default(),
            reduce_only: Default::default(),
            open_only: Default::default(),
        }
    }
}
