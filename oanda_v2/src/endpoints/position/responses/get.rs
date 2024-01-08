use crate::definitions::transaction_id::TransactionID;
use crate::endpoints::position::Position;
/// The Position is provided.
#[derive(Serialize, Deserialize)]
pub struct Get200 {
    /// The requested Position.
    position: Option<Position>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Get200 {
    fn default() -> Self {
        Self {
            position: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
