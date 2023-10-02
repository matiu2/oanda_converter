use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct MarketOrderTradeClose {
    /// The ID of the Trade requested to be closed
    tradeID: Option<TradeID>,
    /// The client ID of the Trade requested to be closed
    clientTradeID: Option<string>,
    /// Indication of how much of the Trade to close. Either “ALL”, or a DecimalNumber reflection a partial close of the Trade.
    units: Option<string>,
}
