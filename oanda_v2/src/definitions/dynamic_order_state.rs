use crate::definitions::order_id::OrderID;
use crate::definitions::price_value::PriceValue;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct DynamicOrderState {
    /// The Order’s ID.
    id: Option<OrderID>,
    /// The Order’s calculated trailing stop value.
    trailing_stop_value: Option<PriceValue>,
    /// The distance between the Trailing Stop Loss Order’s
    /// trailingStopValue and the current Market Price. This
    /// represents the distance (in price units) of the Order from
    /// a triggering price. If the distance could not be determined,
    /// this value will not be set.
    trigger_distance: Option<PriceValue>,
    /// True if an exact trigger distance could be calculated. If
    /// false, it means the provided trigger distance is a best
    /// estimate. If the distance could not be determined, this
    /// value will not be set.
    is_trigger_distance_exact: Option<boolean>,
}
impl Default for DynamicOrderState {
    fn default() -> Self {
        Self {
            id: Default::default(),
            trailing_stop_value: Default::default(),
            trigger_distance: Default::default(),
            is_trigger_distance_exact: Default::default(),
        }
    }
}
