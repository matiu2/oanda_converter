use crate::chrono::DateTime;
use crate::definitions::time_in_force::TimeInForce;
use crate::definitions::client_extensions::ClientExtensions;
use crate::definitions::decimal_number::DecimalNumber;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct TrailingStopLossDetails {
    /// The distance (in price units) from the Trade’s fill price
    /// that the Trailing Stop Loss Order will be triggered at.
    distance: Option<DecimalNumber>,
    /// The time in force for the created Trailing Stop Loss Order.
    /// This may only be GTC, GTD or GFD.
    #[serde_inline_default("GTC")]
    time_in_force: TimeInForce,
    /// The date when the Trailing Stop Loss Order will be cancelled
    /// on if timeInForce is GTD.
    gtd_time: Option<DateTime>,
    /// The Client Extensions to add to the Trailing Stop Loss Order
    /// when created.
    client_extensions: Option<ClientExtensions>,
}
impl Default for TrailingStopLossDetails {
    fn default() -> Self {
        Self {
            distance: Default::default(),
            time_in_force: "GTC",
            gtd_time: Default::default(),
            client_extensions: Default::default(),
        }
    }
}
