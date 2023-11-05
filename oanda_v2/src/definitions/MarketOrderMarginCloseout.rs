use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct MarketOrderMarginCloseout {
    /// The reason the Market Order was created to perform a margin
    /// closeout
    reason: Option<MarketOrderMarginCloseoutReason>,
}
