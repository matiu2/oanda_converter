use definitions::position_book_bucket::PositionBookBucket;
use definitions::instrument_name::InstrumentName;
use chrono::DateTime;
use definitions::price_value::PriceValue;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct PositionBook {
    /// The position book’s instrument
    instrument: Option<InstrumentName>,
    /// The time when the position book snapshot was created
    time: Option<DateTime>,
    /// The price (midpoint) for the position book’s instrument at
    /// the time of the position book snapshot
    price: Option<PriceValue>,
    /// The price width for each bucket. Each bucket covers the
    /// price range from the bucket’s price to the bucket’s price
    /// + bucketWidth.
    bucket_width: Option<PriceValue>,
    /// The partitioned position book, divided into buckets using
    /// a default bucket width. These buckets are only provided for
    /// price ranges which actually contain order or position data.
    buckets: Vec<PositionBookBucket>,
}
impl Default for PositionBook {
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
