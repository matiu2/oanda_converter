use crate::definitions::trade_id::TradeID;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct MarketOrderTradeClose {
    /// The ID of the Trade requested to be closed
    trade_id: Option<TradeID>,
    /// The client ID of the Trade requested to be closed
    client_trade_id: Option<String>,
    /// Indication of how much of the Trade to close. Either “ALL”,
    /// or a DecimalNumber reflection a partial close of the Trade.
    units: Option<String>,
}
impl Default for MarketOrderTradeClose {
    fn default() -> Self {
        Self {
            trade_id: Default::default(),
            client_trade_id: Default::default(),
            units: Default::default(),
        }
    }
}
