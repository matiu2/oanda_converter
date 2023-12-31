use definitions::transaction_id::TransactionID;
use endpoints::position::Position;
use serde::{Serialize, Deserialize};
/// The Account’s Positions are provided.
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Positions200 {
    /// The list of Account Positions.
    positions: Vec<Position>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
