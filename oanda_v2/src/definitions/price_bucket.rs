use crate::definitions::price_value::PriceValue;
use serde_inline_default::serde_inline_default;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct PriceBucket {
    /// The Price offered by the PriceBucket
    price: Option<PriceValue>,
    /// The amount of liquidity offered by the PriceBucket
    liquidity: Option<Decimal>,
}
impl Default for PriceBucket {
    fn default() -> Self {
        Self {
            price: Default::default(),
            liquidity: Default::default(),
        }
    }
}
