use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct MarketOrderMarginCloseout {
    /// The reason the Market Order was created to perform a margin
    /// closeout
    reason: Option<MarketOrderMarginCloseoutReason>,
}
impl Default for MarketOrderMarginCloseout {
    fn default() -> Self {
        Self { reason: Default::default() }
    }
}
