use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
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
    gtd_time: Option<DateTime<Utc>>,
    /// The Client Extensions to add to the Stop Loss Order when
    /// created.
    client_extensions: Option<ClientExtensions>,
}
impl Default for StopLossDetails {
    fn default() -> Self {
        Self {
            price: Default::default(),
            distance: Default::default(),
            time_in_force: "GTC",
            gtd_time: Default::default(),
            client_extensions: Default::default(),
        }
    }
}
