use definitions::instrument_name::InstrumentName;
use definitions::candlestick::Candlestick;
use definitions::candlestick_granularity::CandlestickGranularity;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
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
        use Default::default;
        Self {
            instrument: default(),
            granularity: default(),
            candles: default(),
        }
    }
}
