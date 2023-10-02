use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct StopLossDetails {
    /// The price that the Stop Loss Order will be triggered at. Only one of the price and distance fields may be specified.
    price: Option<PriceValue>,
    /// Specifies the distance (in price units) from the Trade’s open price to use as the Stop Loss Order price. Only one of the distance and price fields may be specified.
    distance: Option<DecimalNumber>,
    /// The time in force for the created Stop Loss Order. This may only be GTC, GTD or GFD.
    #[serde(default = "GTC")]
    timeInForce: TimeInForce,
    /// The date when the Stop Loss Order will be cancelled on if timeInForce is GTD.
    gtdTime: Option<DateTime>,
    /// The Client Extensions to add to the Stop Loss Order when created.
    clientExtensions: Option<ClientExtensions>,
}
