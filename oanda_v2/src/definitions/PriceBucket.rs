use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct PriceBucket {
    /// The Price offered by the PriceBucket
    price: Option<PriceValue>,
    /// The amount of liquidity offered by the PriceBucket
    liquidity: Option<Decimal>,
}
