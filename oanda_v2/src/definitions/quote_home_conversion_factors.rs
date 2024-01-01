use definitions::decimal_number::DecimalNumber;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct QuoteHomeConversionFactors {
    /// The factor used to convert a positive amount of the Price’s
    /// Instrument’s quote currency into a positive amount of
    /// the Account’s home currency. Conversion is performed by
    /// multiplying the quote units by the conversion factor.
    positive_units: Option<DecimalNumber>,
    /// The factor used to convert a negative amount of the Price’s
    /// Instrument’s quote currency into a negative amount of
    /// the Account’s home currency. Conversion is performed by
    /// multiplying the quote units by the conversion factor.
    negative_units: Option<DecimalNumber>,
}
impl Default for QuoteHomeConversionFactors {
    fn default() -> Self {
        use Default::default;
        Self {
            positive_units: default(),
            negative_units: default(),
        }
    }
}
