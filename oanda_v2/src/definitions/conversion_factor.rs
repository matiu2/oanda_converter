use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct ConversionFactor {
    /// The factor by which to multiply the amount in the given
    /// currency to obtain the amount in the home currency of the
    /// Account.
    factor: Option<DecimalNumber>,
}
impl Default for ConversionFactor {
    fn default() -> Self {
        Self { factor: Default::default() }
    }
}
