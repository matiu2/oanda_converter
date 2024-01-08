use crate::endpoints::order::Order;
use crate::definitions::transaction_id::TransactionID;
/// List of pending Orders for the Account
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
        Self {
            orders: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
