/// The Account’s list of open Trades is provided
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct OpenTrades {
    /// The Account’s list of open Trades
    trades: Vec<Trade>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for OpenTrades {
    fn default() -> Self {
        Self {
            trades: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
