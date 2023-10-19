use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct ConversionFactor {
    /// The factor by which to multiply the amount in the given currency to obtain the amount in the home currency of the Account.
    factor: Option<DecimalNumber>,
}
