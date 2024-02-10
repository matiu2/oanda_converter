use crate::endpoints::order::Order;
use crate::definitions::transaction_id::TransactionID;
/// The list of Orders requested
#[derive(Serialize, Deserialize)]
pub struct Orders {
    /// The list of Order detail objects
    orders: Vec<Order>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Orders {
    fn default() -> Self {
        Self {
            orders: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
