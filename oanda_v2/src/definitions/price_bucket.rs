use definitions::price_value::PriceValue;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct PriceBucket {
    /// The Price offered by the PriceBucket
    price: Option<PriceValue>,
    /// The amount of liquidity offered by the PriceBucket
    liquidity: Option<Decimal>,
}
