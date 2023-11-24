use serde::{Serialize, Deserialize};
/// The current state of the Order.
#[derive(Deserialize, Serialize)]
#[rename_all("SCREAMING_SNAKE_CASE")]
pub enum OrderState {
    /// The Order is currently pending execution
    Pending,
    /// The Order has been filled
    Filled,
    /// The Order has been triggered
    Triggered,
    /// The Order has been cancelled
    Cancelled,
}
