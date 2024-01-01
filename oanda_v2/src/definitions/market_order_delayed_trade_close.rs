use definitions::trade_id::TradeID;
use definitions::transaction_id::TransactionID;
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
impl Default for MarketOrderDelayedTradeClose {
    fn default() -> Self {
        use Default::default;
        Self {
            trade_id: default(),
            client_trade_id: default(),
            source_transaction_id: default(),
        }
    }
}
