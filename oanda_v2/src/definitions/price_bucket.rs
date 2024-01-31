use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
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
