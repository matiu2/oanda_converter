use serde::{Serialize, Deserialize};
/// The details of the Order requested
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Get200 {
    /// The details of the Order requested
    order: Option<Order>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
