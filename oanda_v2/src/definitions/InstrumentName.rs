/// Instrument name identifier. Used by clients to refer to an Instrument.
///
/// A string containing the base currency and quote currency delimited by a “_”.
struct InstrumentName(String);
impl std::ops::Deref for InstrumentName {
    type Target = &str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
