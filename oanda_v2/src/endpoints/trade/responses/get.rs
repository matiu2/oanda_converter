use crate::definitions::transaction_id::TransactionID;
use crate::endpoints::trade::Trade;
/// The details for the requested Trade is provided
#[derive(Serialize, Deserialize)]
pub struct Get {
    /// The details of the requested trade
    trade: Option<Trade>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Get {
    fn default() -> Self {
        Self {
            trade: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
