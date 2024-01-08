use crate::endpoints::trade::Trade;
use crate::definitions::transaction_id::TransactionID;
/// The Account’s list of open Trades is provided
#[derive(Serialize, Deserialize)]
pub struct OpenTrades200 {
    /// The Account’s list of open Trades
    trades: Vec<Trade>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for OpenTrades200 {
    fn default() -> Self {
        Self {
            trades: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
