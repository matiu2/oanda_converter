use crate::definitions::candlestick_response::CandlestickResponse;
/// Pricing information has been successfully provided.
#[derive(Serialize, Deserialize)]
pub struct Latest200 {
    /// The latest candle sticks.
    latest_candles: Vec<CandlestickResponse>,
}
impl Default for Latest200 {
    fn default() -> Self {
        Self {
            latest_candles: Default::default(),
        }
    }
}
