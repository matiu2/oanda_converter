use serde::{Serialize, Deserialize};
/// The Price component(s) to get candlestick data for.
///
/// Can contain any combination of the characters “M” (midpoint
/// candles) “B” (bid candles) and “A” (ask candles).
struct PricingComponent(String);
impl std::ops::Deref for PricingComponent {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
