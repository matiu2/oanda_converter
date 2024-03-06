use crate::definitions::decimal_number::DecimalNumber;
use serde_inline_default::serde_inline_default;
use crate::definitions::currency::Currency;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct HomeConversions {
    /// The currency to be converted into the home currency.
    currency: Option<Currency>,
    /// The factor used to convert any gains for an Account in the
    /// specified currency into the Account’s home currency. This
    /// would include positive realized P/L and positive financing
    /// amounts. Conversion is performed by multiplying the positive
    /// P/L by the conversion factor.
    account_gain: Option<DecimalNumber>,
    /// The factor used to convert any losses for an Account in the
    /// specified currency into the Account’s home currency. This
    /// would include negative realized P/L and negative financing
    /// amounts. Conversion is performed by multiplying the positive
    /// P/L by the conversion factor.
    account_loss: Option<DecimalNumber>,
    /// The factor used to convert a Position or Trade Value in
    /// the specified currency into the Account’s home currency.
    /// Conversion is performed by multiplying the Position or Trade
    /// Value by the conversion factor.
    position_value: Option<DecimalNumber>,
}
impl Default for HomeConversions {
    fn default() -> Self {
        Self {
            currency: Default::default(),
            account_gain: Default::default(),
            account_loss: Default::default(),
            position_value: Default::default(),
        }
    }
}
