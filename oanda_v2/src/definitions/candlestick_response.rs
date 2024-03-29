use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct CandlestickResponse {
    /// The instrument whose Prices are represented by the
    /// candlesticks.
    instrument: Option<InstrumentName>,
    /// The granularity of the candlesticks provided.
    granularity: Option<CandlestickGranularity>,
    /// The list of candlesticks that satisfy the request.
    candles: Vec<Candlestick>,
}
impl Default for CandlestickResponse {
    fn default() -> Self {
        Self {
            instrument: Default::default(),
            granularity: Default::default(),
            candles: Default::default(),
        }
    }
}
