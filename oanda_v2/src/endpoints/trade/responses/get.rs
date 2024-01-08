use crate::endpoints::trade::Trade;
use crate::definitions::transaction_id::TransactionID;
/// The details for the requested Trade is provided
#[derive(Serialize, Deserialize)]
pub struct Get200 {
    /// The details of the requested trade
    trade: Option<Trade>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for Get200 {
    fn default() -> Self {
        Self {
            trade: Default::default(),
            last_transaction_id: Default::default(),
        }
    }
}
