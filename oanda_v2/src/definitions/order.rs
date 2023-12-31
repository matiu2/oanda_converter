use chrono::DateTime;
use definitions::order_state::OrderState;
use definitions::order_id::OrderID;
use definitions::client_extensions::ClientExtensions;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Order {
    /// The Order’s identifier, unique within the Order’s Account.
    id: Option<OrderID>,
    /// The time when the Order was created.
    create_time: Option<DateTime>,
    /// The current state of the Order.
    state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or
    /// delete clientExtensions if your account is associated with
    /// MT4.
    client_extensions: Option<ClientExtensions>,
}
