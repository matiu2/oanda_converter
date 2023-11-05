/// An instrument name, a granularity, and a price component to
/// get candlestick data for.
///
/// A string containing the following, all delimited by “:”
/// characters: 1) InstrumentName 2) CandlestickGranularity 3)
/// PricingComponent e.g. EUR_USD:S10:BM
struct CandleSpecification(String);
impl std::ops::Deref for CandleSpecification {
    type Target = &str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
