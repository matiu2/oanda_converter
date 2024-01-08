use crate::definitions::instrument_name::InstrumentName;
use crate::definitions::candlestick::Candlestick;
use crate::definitions::candlestick_granularity::CandlestickGranularity;
/// Pricing information has been successfully provided.
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
impl Default for Candles200 {
    fn default() -> Self {
        Self {
            instrument: Default::default(),
            granularity: Default::default(),
            candles: Default::default(),
        }
    }
}
