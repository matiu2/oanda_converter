use serde_inline_default::serde_inline_default;
use crate::definitions::transaction_id::TransactionID;
use chrono::Utc;
use chrono::DateTime;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct TransactionHeartbeat {
    /// The string “HEARTBEAT”
    #[serde_inline_default("HEARTBEAT")]
    r#type: String,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
    /// The date/time when the TransactionHeartbeat was created.
    time: Option<DateTime<Utc>>,
}
impl Default for TransactionHeartbeat {
    fn default() -> Self {
        Self {
            r#type: "HEARTBEAT",
            last_transaction_id: Default::default(),
            time: Default::default(),
        }
    }
}
