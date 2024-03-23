/// Pricing information has been successfully provided.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Latest {
    /// The latest candle sticks.
    latest_candles: Vec<CandlestickResponse>,
}
impl Default for Latest {
    fn default() -> Self {
        Self {
            latest_candles: Default::default(),
        }
    }
}
