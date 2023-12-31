use definitions::candlestick_response::CandlestickResponse;
use serde::{Serialize, Deserialize};
/// Pricing information has been successfully provided.
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Latest200 {
    /// The latest candle sticks.
    latest_candles: Vec<CandlestickResponse>,
}
