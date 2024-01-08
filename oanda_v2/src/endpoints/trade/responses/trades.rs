use crate::endpoints::trade::Trade;
use crate::definitions::transaction_id::TransactionID;
/// The list of Trades requested
#[derive(Serialize, Deserialize)]
pub struct Trades200 {
    /// The list of Trade detail objects
    trades: Vec<Trade>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Trades200 {
    fn default() -> Self {
        Self {
            trades: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
