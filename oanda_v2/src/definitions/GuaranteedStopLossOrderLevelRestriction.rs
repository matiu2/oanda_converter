use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct GuaranteedStopLossOrderLevelRestriction {
    /// Applies to Trades with a guaranteed Stop Loss Order attached for the specified Instrument. This is the total allowed Trade volume that can exist within the priceRange based on the trigger prices of the guaranteed Stop Loss Orders.
    volume: Option<DecimalNumber>,
    /// The price range the volume applies to. This value is in price units.
    price_range: Option<DecimalNumber>,
}