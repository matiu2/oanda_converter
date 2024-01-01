use definitions::candlestick_data::CandlestickData;
use chrono::DateTime;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Candlestick {
    /// The start time of the candlestick
    time: Option<DateTime>,
    /// The candlestick data based on bids. Only provided if bid-
    /// based candles were requested.
    bid: Option<CandlestickData>,
    /// The candlestick data based on asks. Only provided if ask-
    /// based candles were requested.
    ask: Option<CandlestickData>,
    /// The candlestick data based on midpoints. Only provided if
    /// midpoint-based candles were requested.
    mid: Option<CandlestickData>,
    /// The number of prices created during the time-range
    /// represented by the candlestick.
    volume: Option<integer>,
    /// A flag indicating if the candlestick is complete. A complete
    /// candlestick is one whose ending time is not in the future.
    complete: Option<boolean>,
}
impl Default for Candlestick {
    fn default() -> Self {
        use Default::default;
        Self {
            time: default(),
            bid: default(),
            ask: default(),
            mid: default(),
            volume: default(),
            complete: default(),
        }
    }
}
