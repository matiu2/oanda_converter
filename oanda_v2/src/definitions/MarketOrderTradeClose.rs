use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct MarketOrderTradeClose {
    /// The ID of the Trade requested to be closed
    trade_id: Option<TradeID>,
    /// The client ID of the Trade requested to be closed
    client_trade_id: Option<string>,
    /// Indication of how much of the Trade to close. Either “ALL”, or a DecimalNumber reflection a partial close of the Trade.
    units: Option<string>,
}
