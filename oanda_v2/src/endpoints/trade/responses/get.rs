use definitions::transaction_id::TransactionID;
use endpoints::trade::Trade;
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
