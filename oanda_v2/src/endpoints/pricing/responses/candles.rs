use serde::{Serialize, Deserialize};
/// Pricing information has been successfully provided.
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Candles200 {
    /// The instrument whose Prices are represented by the
    /// candlesticks.
    instrument: Option<InstrumentName>,
    /// The granularity of the candlesticks provided.
    granularity: Option<CandlestickGranularity>,
    /// The list of candlesticks that satisfy the request.
    candles: Vec<Candlestick>,
}
