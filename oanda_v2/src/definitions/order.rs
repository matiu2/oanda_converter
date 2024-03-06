use chrono::DateTime;
use serde_inline_default::serde_inline_default;
use crate::definitions::order_state::OrderState;
use crate::definitions::order_id::OrderID;
use crate::endpoints::trade::responses::client_extensions::ClientExtensions;
use chrono::Utc;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Order {
    /// The Order’s identifier, unique within the Order’s Account.
    id: Option<OrderID>,
    /// The time when the Order was created.
    create_time: Option<DateTime<Utc>>,
    /// The current state of the Order.
    state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or
    /// delete clientExtensions if your account is associated with
    /// MT4.
    client_extensions: Option<ClientExtensions>,
}
impl Default for Order {
    fn default() -> Self {
        Self {
            id: Default::default(),
            create_time: Default::default(),
            state: Default::default(),
            client_extensions: Default::default(),
        }
    }
}
