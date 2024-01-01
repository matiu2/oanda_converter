use definitions::guaranteed_stop_loss_order_level_restriction::GuaranteedStopLossOrderLevelRestriction;
use definitions::decimal_number::DecimalNumber;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct GuaranteedStopLossOrderEntryData {
    /// The minimum distance allowed between the Trade’s fill price
    /// and the configured price for guaranteed Stop Loss Orders
    /// created for this instrument. Specified in price units.
    minimum_distance: Option<DecimalNumber>,
    /// The amount that is charged to the account if a guaranteed
    /// Stop Loss Order is triggered and filled. The value is in
    /// price units and is charged for each unit of the Trade.
    premium: Option<DecimalNumber>,
    /// The guaranteed Stop Loss Order level restriction for this
    /// instrument.
    level_restriction: Option<GuaranteedStopLossOrderLevelRestriction>,
}
impl Default for GuaranteedStopLossOrderEntryData {
    fn default() -> Self {
        use Default::default;
        Self {
            minimum_distance: default(),
            premium: default(),
            level_restriction: default(),
        }
    }
}
