use crate::endpoints::position::Position;
use crate::definitions::transaction_id::TransactionID;
/// The Account’s Positions are provided.
#[derive(Serialize, Deserialize)]
pub struct Positions200 {
    /// The list of Account Positions.
    positions: Vec<Position>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Positions200 {
    fn default() -> Self {
        Self {
            positions: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
