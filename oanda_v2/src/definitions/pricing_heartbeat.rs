use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct PricingHeartbeat {
    /// The string “HEARTBEAT”
    #[serde(default = "HEARTBEAT")]
    r#type: String,
    /// The date/time when the Heartbeat was created.
    time: Option<DateTime>,
}
