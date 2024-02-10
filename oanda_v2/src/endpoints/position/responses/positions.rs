use crate::definitions::transaction_id::TransactionID;
use serde_inline_default::serde_inline_default;
use crate::endpoints::position::Position;
/// The Account’s Positions are provided.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Positions {
    /// The list of Account Positions.
    positions: Vec<Position>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Positions {
    fn default() -> Self {
        Self {
            positions: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
