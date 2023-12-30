use lib::endpoints::position;
use serde::{Serialize, Deserialize};
/// The Position is provided.
#[derive(Serialize, Deserialize)]
struct Get200 {
    /// The requested Position.
    position: Option<Position>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
