use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
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
        Self {
            trade_id: Default::default(),
            client_trade_id: Default::default(),
            source_transaction_id: Default::default(),
        }
    }
}
