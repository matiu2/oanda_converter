use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct TrailingStopLossDetails {
    /// The distance (in price units) from the Trade’s fill price that the Trailing Stop Loss Order will be triggered at.
    distance: Option<DecimalNumber>,
    /// The time in force for the created Trailing Stop Loss Order. This may only be GTC, GTD or GFD.
    #[serde(default = "GTC")]
    timeInForce: TimeInForce,
    /// The date when the Trailing Stop Loss Order will be cancelled on if timeInForce is GTD.
    gtdTime: Option<DateTime>,
    /// The Client Extensions to add to the Trailing Stop Loss Order when created.
    clientExtensions: Option<ClientExtensions>,
}
