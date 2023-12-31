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
