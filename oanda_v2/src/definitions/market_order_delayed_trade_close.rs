use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct MarketOrderDelayedTradeClose {
    /// The ID of the Trade being closed
    trade_id: Option<TradeID>,
    /// The Client ID of the Trade being closed
    client_trade_id: Option<TradeID>,
    /// The Transaction ID of the DelayedTradeClosure transaction to
    /// which this Delayed Trade Close belongs to
    source_transaction_id: Option<TransactionID>,
}
