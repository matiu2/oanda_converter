use serde::{Serialize, Deserialize};
/// List of pending Orders for the Account
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct PendingOrders200 {
    /// The list of pending Order details
    orders: Vec<Order>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
