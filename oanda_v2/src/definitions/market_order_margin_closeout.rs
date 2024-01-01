use definitions::market_order_margin_closeout_reason::MarketOrderMarginCloseoutReason;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct MarketOrderMarginCloseout {
    /// The reason the Market Order was created to perform a margin
    /// closeout
    reason: Option<MarketOrderMarginCloseoutReason>,
}
impl Default for MarketOrderMarginCloseout {
    fn default() -> Self {
        use Default::default;
        Self { reason: default() }
    }
}
