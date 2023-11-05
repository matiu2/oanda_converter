use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct TakeProfitDetails {
    /// The price that the Take Profit Order will be triggered at.
    /// Only one of the price and distance fields may be specified.
    price: Option<PriceValue>,
    /// The time in force for the created Take Profit Order. This
    /// may only be GTC, GTD or GFD.
    #[serde(default = "GTC")]
    time_in_force: TimeInForce,
    /// The date when the Take Profit Order will be cancelled on if
    /// timeInForce is GTD.
    gtd_time: Option<DateTime>,
    /// The Client Extensions to add to the Take Profit Order when
    /// created.
    client_extensions: Option<ClientExtensions>,
}
