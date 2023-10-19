use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct PricingHeartbeat {
    /// The string “HEARTBEAT”
    #[serde(default = "HEARTBEAT")]
    r#type: string,
    /// The date/time when the Heartbeat was created.
    time: Option<DateTime>,
}
