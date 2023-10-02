use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct MarketOrderDelayedTradeClose {
    /// The ID of the Trade being closed
    tradeID: Option<TradeID>,
    /// The Client ID of the Trade being closed
    clientTradeID: Option<TradeID>,
    /// The Transaction ID of the DelayedTradeClosure transaction to which this Delayed Trade Close belongs to
    sourceTransactionID: Option<TransactionID>,
}
