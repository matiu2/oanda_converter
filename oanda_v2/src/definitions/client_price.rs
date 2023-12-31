use chrono::DateTime;
use definitions::instrument_name::InstrumentName;
use definitions::price_bucket::PriceBucket;
use definitions::price_value::PriceValue;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct ClientPrice {
    /// The string “PRICE”. Used to identify the a Price object when
    /// found in a stream.
    #[serde(default = "PRICE")]
    r#type: String,
    /// The Price’s Instrument.
    instrument: Option<InstrumentName>,
    /// The date/time when the Price was created
    time: Option<DateTime>,
    /// Flag indicating if the Price is tradeable or not
    tradeable: Option<boolean>,
    /// The list of prices and liquidity available on the
    /// Instrument’s bid side. It is possible for this list to be
    /// empty if there is no bid liquidity currently available for
    /// the Instrument in the Account.
    bids: Vec<PriceBucket>,
    /// The list of prices and liquidity available on the
    /// Instrument’s ask side. It is possible for this list to be
    /// empty if there is no ask liquidity currently available for
    /// the Instrument in the Account.
    asks: Vec<PriceBucket>,
    /// The closeout bid Price. This Price is used when a bid is
    /// required to closeout a Position (margin closeout or manual)
    /// yet there is no bid liquidity. The closeout bid is never
    /// used to open a new position.
    closeout_bid: Option<PriceValue>,
    /// The closeout ask Price. This Price is used when a ask is
    /// required to closeout a Position (margin closeout or manual)
    /// yet there is no ask liquidity. The closeout ask is never
    /// used to open a new position.
    closeout_ask: Option<PriceValue>,
}
