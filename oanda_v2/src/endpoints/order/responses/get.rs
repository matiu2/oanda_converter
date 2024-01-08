use crate::endpoints::order::Order;
use crate::definitions::transaction_id::TransactionID;
/// The details of the Order requested
#[derive(Serialize, Deserialize)]
pub struct Get200 {
    /// The details of the Order requested
    order: Option<Order>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Get200 {
    fn default() -> Self {
        Self {
            order: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
