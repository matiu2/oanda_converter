use serde::{Serialize, Deserialize};
/// The details of the Order requested
#[derive(Serialize, Deserialize)]
struct Get200 {
    /// The details of the Order requested
    order: Option<Order>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
