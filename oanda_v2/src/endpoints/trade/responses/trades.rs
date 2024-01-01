use definitions::transaction_id::TransactionID;
use endpoints::trade::Trade;
use serde::{Serialize, Deserialize};
/// The list of Trades requested
use serde::{Serialize, Deserialize};
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
        use Default::default;
        Self {
            trades: default(),
            last_transaction_id: default(),
        }
    }
}
