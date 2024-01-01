use chrono::DateTime;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct PricingHeartbeat {
    /// The string “HEARTBEAT”
    #[serde_inline_default("HEARTBEAT")]
    r#type: String,
    /// The date/time when the Heartbeat was created.
    time: Option<DateTime>,
}
impl Default for PricingHeartbeat {
    fn default() -> Self {
        use Default::default;
        Self {
            r#type: "HEARTBEAT",
            time: default(),
        }
    }
}
