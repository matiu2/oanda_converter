use crate::definitions::order_book_bucket::OrderBookBucket;
use crate::definitions::price_value::PriceValue;
use serde_inline_default::serde_inline_default;
use crate::definitions::instrument_name::InstrumentName;
use chrono::DateTime;
use chrono::Utc;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct OrderBook {
    /// The order book’s instrument
    instrument: Option<InstrumentName>,
    /// The time when the order book snapshot was created.
    time: Option<DateTime<Utc>>,
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
        Self {
            instrument: Default::default(),
            time: Default::default(),
            price: Default::default(),
            bucket_width: Default::default(),
            buckets: Default::default(),
        }
    }
}
