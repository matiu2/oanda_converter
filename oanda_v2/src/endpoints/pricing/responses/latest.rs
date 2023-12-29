use serde::{Serialize, Deserialize};
/// Pricing information has been successfully provided.
#[derive(Serialize, Deserialize)]
struct Latest200 {
    /// The latest candle sticks.
    latest_candles: Vec<CandlestickResponse>,
}
