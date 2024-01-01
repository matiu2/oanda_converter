use definitions::order_id::OrderID;
use chrono::DateTime;
use definitions::client_extensions::ClientExtensions;
use definitions::order_state::OrderState;
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
impl Default for Order {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            create_time: default(),
            state: default(),
            client_extensions: default(),
        }
    }
}
