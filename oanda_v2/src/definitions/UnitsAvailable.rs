use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct UnitsAvailable {
    /// The number of units that are available to be traded using an Order with a positionFill option of “DEFAULT”. For an Account with hedging enabled, this value will be the same as the “OPEN_ONLY” value. For an Account without hedging enabled, this value will be the same as the “REDUCE_FIRST” value.
    default: Option<UnitsAvailableDetails>,
    /// The number of units that may are available to be traded with an Order with a positionFill option of “REDUCE_FIRST”.
    reduceFirst: Option<UnitsAvailableDetails>,
    /// The number of units that may are available to be traded with an Order with a positionFill option of “REDUCE_ONLY”.
    reduceOnly: Option<UnitsAvailableDetails>,
    /// The number of units that may are available to be traded with an Order with a positionFill option of “OPEN_ONLY”.
    openOnly: Option<UnitsAvailableDetails>,
}
