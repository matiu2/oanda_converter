use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct TransactionHeartbeat {
    /// The string “HEARTBEAT”
    #[serde(default = "HEARTBEAT")]
    r#type: string,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
    /// The date/time when the TransactionHeartbeat was created.
    time: Option<DateTime>,
}
