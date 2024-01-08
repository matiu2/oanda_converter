use crate::definitions::transaction_id::TransactionID;
use crate::endpoints::order::Order;
/// The list of Orders requested
#[derive(Serialize, Deserialize)]
pub struct Orders200 {
    /// The list of Order detail objects
    orders: Vec<Order>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Orders200 {
    fn default() -> Self {
        Self {
            orders: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
