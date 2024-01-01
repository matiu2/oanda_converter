use definitions::instrument_name::InstrumentName;
use definitions::order_book_bucket::OrderBookBucket;
use definitions::price_value::PriceValue;
use chrono::DateTime;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct OrderBook {
    /// The order book’s instrument
    instrument: Option<InstrumentName>,
    /// The time when the order book snapshot was created.
    time: Option<DateTime>,
    /// The price (midpoint) for the order book’s instrument at the
    /// time of the order book snapshot
    price: Option<PriceValue>,
    /// The price width for each bucket. Each bucket covers the
    /// price range from the bucket’s price to the bucket’s price
    /// + bucketWidth.
    bucket_width: Option<PriceValue>,
    /// The partitioned order book, divided into buckets using a
    /// default bucket width. These buckets are only provided for
    /// price ranges which actually contain order or position data.
    buckets: Vec<OrderBookBucket>,
}
impl Default for OrderBook {
    fn default() -> Self {
        use Default::default;
        Self {
            instrument: default(),
            time: default(),
            price: default(),
            bucket_width: default(),
            buckets: default(),
        }
    }
}
