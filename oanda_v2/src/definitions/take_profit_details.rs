use serde_inline_default::serde_inline_default;
use crate::definitions::time_in_force::TimeInForce;
use chrono::DateTime;
use crate::endpoints::trade::responses::client_extensions::ClientExtensions;
use chrono::Utc;
use crate::definitions::price_value::PriceValue;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct TakeProfitDetails {
    /// The price that the Take Profit Order will be triggered at.
    /// Only one of the price and distance fields may be specified.
    price: Option<PriceValue>,
    /// The time in force for the created Take Profit Order. This
    /// may only be GTC, GTD or GFD.
    #[serde_inline_default("GTC")]
    time_in_force: TimeInForce,
    /// The date when the Take Profit Order will be cancelled on if
    /// timeInForce is GTD.
    gtd_time: Option<DateTime<Utc>>,
    /// The Client Extensions to add to the Take Profit Order when
    /// created.
    client_extensions: Option<ClientExtensions>,
}
impl Default for TakeProfitDetails {
    fn default() -> Self {
        Self {
            price: Default::default(),
            time_in_force: "GTC",
            gtd_time: Default::default(),
            client_extensions: Default::default(),
        }
    }
}
