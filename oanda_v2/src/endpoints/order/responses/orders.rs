use serde::{Serialize, Deserialize};
/// The list of Orders requested
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Orders200 {
    /// The list of Order detail objects
    orders: Vec<Order>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
