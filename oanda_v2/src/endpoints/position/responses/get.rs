use endpoints::position::Position;
use definitions::transaction_id::TransactionID;
use serde::{Serialize, Deserialize};
/// The Position is provided.
use serde::{Serialize, Deserialize};
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
        use Default::default;
        Self {
            position: default(),
            last_transaction_id: default(),
        }
    }
}
