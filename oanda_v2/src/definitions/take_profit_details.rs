use chrono::DateTime;
use definitions::client_extensions::ClientExtensions;
use definitions::price_value::PriceValue;
use definitions::time_in_force::TimeInForce;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
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
    gtd_time: Option<DateTime>,
    /// The Client Extensions to add to the Take Profit Order when
    /// created.
    client_extensions: Option<ClientExtensions>,
}
impl Default for TakeProfitDetails {
    fn default() -> Self {
        use Default::default;
        Self {
            price: default(),
            time_in_force: "GTC",
            gtd_time: default(),
            client_extensions: default(),
        }
    }
}
