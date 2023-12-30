use serde::{Serialize, Deserialize};
/// The details for the requested Trade is provided
#[derive(Serialize, Deserialize)]
struct Get200 {
    /// The details of the requested trade
    trade: Option<Trade>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
