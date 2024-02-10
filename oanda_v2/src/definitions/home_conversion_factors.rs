use serde_inline_default::serde_inline_default;
use crate::definitions::conversion_factor::ConversionFactor;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
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
impl Default for HomeConversionFactors {
    fn default() -> Self {
        Self {
            gain_quote_home: Default::default(),
            loss_quote_home: Default::default(),
            gain_base_home: Default::default(),
            loss_base_home: Default::default(),
        }
    }
}
