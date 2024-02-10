use crate::endpoints::order::Order;
use crate::definitions::transaction_id::TransactionID;
use serde_inline_default::serde_inline_default;
/// The details of the Order requested
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Get {
    /// The details of the Order requested
    order: Option<Order>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Get {
    fn default() -> Self {
        Self {
            order: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
