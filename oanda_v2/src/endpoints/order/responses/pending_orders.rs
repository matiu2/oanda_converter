use definitions::transaction_id::TransactionID;
use endpoints::order::Order;
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
impl Default for PendingOrders200 {
    fn default() -> Self {
        use Default::default;
        Self {
            orders: default(),
            last_transaction_id: default(),
        }
    }
}
