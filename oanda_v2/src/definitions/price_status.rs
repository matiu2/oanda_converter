use serde::{Serialize, Deserialize};
/// The status of the Price.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceStatus {
    /// The Instrument’s price is tradeable.
    Tradeable,
    /// The Instrument’s price is not tradeable.
    NonTradeable,
    /// The Instrument of the price is invalid or there is no valid
    /// Price for the Instrument.
    Invalid,
}
