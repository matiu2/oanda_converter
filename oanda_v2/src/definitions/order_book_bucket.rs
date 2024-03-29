use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct OrderBookBucket {
    /// The lowest price (inclusive) covered by the bucket. The
    /// bucket covers the price range from the price to price + the
    /// order book’s bucketWidth.
    price: Option<PriceValue>,
    /// The percentage of the total number of orders represented by
    /// the long orders found in this bucket.
    long_count_percent: Option<DecimalNumber>,
    /// The percentage of the total number of orders represented by
    /// the short orders found in this bucket.
    short_count_percent: Option<DecimalNumber>,
}
impl Default for OrderBookBucket {
    fn default() -> Self {
        Self {
            price: Default::default(),
            long_count_percent: Default::default(),
            short_count_percent: Default::default(),
        }
    }
}
