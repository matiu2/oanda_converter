use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct PositionBookBucket {
    /// The lowest price (inclusive) covered by the bucket. The bucket covers the price range from the price to price + the position book’s bucketWidth.
    price: Option<PriceValue>,
    /// The percentage of the total number of positions represented by the long positions found in this bucket.
    long_count_percent: Option<DecimalNumber>,
    /// The percentage of the total number of positions represented by the short positions found in this bucket.
    short_count_percent: Option<DecimalNumber>,
}
