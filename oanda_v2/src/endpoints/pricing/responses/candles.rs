/// Pricing information has been successfully provided.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Candles {
    /// The instrument whose Prices are represented by the
    /// candlesticks.
    instrument: Option<InstrumentName>,
    /// The granularity of the candlesticks provided.
    granularity: Option<CandlestickGranularity>,
    /// The list of candlesticks that satisfy the request.
    candles: Vec<Candlestick>,
}
impl Default for Candles {
    fn default() -> Self {
        Self {
            instrument: Default::default(),
            granularity: Default::default(),
            candles: Default::default(),
        }
    }
}
