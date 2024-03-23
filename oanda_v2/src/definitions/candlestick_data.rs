use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct CandlestickData {
    /// The first (open) price in the time-range represented by
    /// the candlestick.
    o: Option<PriceValue>,
    /// The highest price in the time-range represented by the
    /// candlestick.
    h: Option<PriceValue>,
    /// The lowest price in the time-range represented by the
    /// candlestick.
    l: Option<PriceValue>,
    /// The last (closing) price in the time-range represented by
    /// the candlestick.
    c: Option<PriceValue>,
}
impl Default for CandlestickData {
    fn default() -> Self {
        Self {
            o: Default::default(),
            h: Default::default(),
            l: Default::default(),
            c: Default::default(),
        }
    }
}
