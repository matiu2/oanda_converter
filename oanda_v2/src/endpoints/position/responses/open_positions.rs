use definitions::transaction_id::TransactionID;
use endpoints::position::Position;
use serde::{Serialize, Deserialize};
/// The Account’s open Positions are provided.
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct OpenPositions200 {
    /// The list of open Positions in the Account.
    positions: Vec<Position>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for OpenPositions200 {
    fn default() -> Self {
        use Default::default;
        Self {
            positions: default(),
            last_transaction_id: default(),
        }
    }
}
