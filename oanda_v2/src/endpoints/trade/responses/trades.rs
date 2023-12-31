use endpoints::trade::Trade;
use definitions::transaction_id::TransactionID;
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
