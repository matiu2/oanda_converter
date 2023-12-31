use serde::{Serialize, Deserialize};
/// The overall behaviour of the Account regarding guaranteed
/// Stop Loss Orders.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GuaranteedStopLossOrderMode {
    /// The Account is not permitted to create guaranteed Stop Loss
    /// Orders.
    Disabled,
    /// The Account is able, but not required to have guaranteed
    /// Stop Loss Orders for open Trades.
    Allowed,
    /// The Account is required to have guaranteed Stop Loss Orders
    /// for all open Trades.
    Required,
}
