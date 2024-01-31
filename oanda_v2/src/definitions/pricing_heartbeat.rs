use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct PricingHeartbeat {
    /// The string “HEARTBEAT”
    #[serde_inline_default("HEARTBEAT")]
    r#type: String,
    /// The date/time when the Heartbeat was created.
    time: Option<DateTime<Utc>>,
}
impl Default for PricingHeartbeat {
    fn default() -> Self {
        Self {
            r#type: "HEARTBEAT",
            time: Default::default(),
        }
    }
}
