use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct HomeConversionFactors {
    /// The ConversionFactor in effect for the Account for
    /// converting any gains realized in Instrument quote units into
    /// units of the Account’s home currency.
    gain_quote_home: Option<ConversionFactor>,
    /// The ConversionFactor in effect for the Account for
    /// converting any losses realized in Instrument quote units
    /// into units of the Account’s home currency.
    loss_quote_home: Option<ConversionFactor>,
    /// The ConversionFactor in effect for the Account for
    /// converting any gains realized in Instrument base units into
    /// units of the Account’s home currency.
    gain_base_home: Option<ConversionFactor>,
    /// The ConversionFactor in effect for the Account for
    /// converting any losses realized in Instrument base units into
    /// units of the Account’s home currency.
    loss_base_home: Option<ConversionFactor>,
}
