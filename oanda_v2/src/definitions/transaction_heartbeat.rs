use definitions::transaction_id::TransactionID;
use chrono::DateTime;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct TransactionHeartbeat {
    /// The string “HEARTBEAT”
    #[serde_inline_default("HEARTBEAT")]
    r#type: String,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
    /// The date/time when the TransactionHeartbeat was created.
    time: Option<DateTime>,
}
impl Default for TransactionHeartbeat {
    fn default() -> Self {
        use Default::default;
        Self {
            r#type: "HEARTBEAT",
            last_transaction_id: default(),
            time: default(),
        }
    }
}
