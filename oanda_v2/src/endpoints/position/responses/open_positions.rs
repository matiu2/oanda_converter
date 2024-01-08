use crate::definitions::transaction_id::TransactionID;
use crate::endpoints::position::Position;
/// The Account’s open Positions are provided.
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
        Self {
            positions: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
