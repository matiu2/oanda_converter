use endpoints::trade::Trade;
use definitions::transaction_id::TransactionID;
use serde::{Serialize, Deserialize};
/// The details for the requested Trade is provided
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Get200 {
    /// The details of the requested trade
    trade: Option<Trade>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Get200 {
    fn default() -> Self {
        use Default::default;
        Self {
            trade: default(),
            last_transaction_id: default(),
        }
    }
}
