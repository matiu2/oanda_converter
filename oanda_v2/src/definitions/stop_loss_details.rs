use definitions::time_in_force::TimeInForce;
use definitions::price_value::PriceValue;
use definitions::client_extensions::ClientExtensions;
use definitions::decimal_number::DecimalNumber;
use chrono::DateTime;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct StopLossDetails {
    /// The price that the Stop Loss Order will be triggered at.
    /// Only one of the price and distance fields may be specified.
    price: Option<PriceValue>,
    /// Specifies the distance (in price units) from the Trade’s
    /// open price to use as the Stop Loss Order price. Only one of
    /// the distance and price fields may be specified.
    distance: Option<DecimalNumber>,
    /// The time in force for the created Stop Loss Order. This may
    /// only be GTC, GTD or GFD.
    #[serde_inline_default("GTC")]
    time_in_force: TimeInForce,
    /// The date when the Stop Loss Order will be cancelled on if
    /// timeInForce is GTD.
    gtd_time: Option<DateTime>,
    /// The Client Extensions to add to the Stop Loss Order when
    /// created.
    client_extensions: Option<ClientExtensions>,
}
impl Default for StopLossDetails {
    fn default() -> Self {
        use Default::default;
        Self {
            price: default(),
            distance: default(),
            time_in_force: "GTC",
            gtd_time: default(),
            client_extensions: default(),
        }
    }
}
