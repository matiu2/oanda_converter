use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct Order {
    /// The Order’s identifier, unique within the Order’s Account.
    id: Option<OrderID>,
    /// The time when the Order was created.
    createTime: Option<DateTime>,
    /// The current state of the Order.
    state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or delete clientExtensions if your account is associated with MT4.
    clientExtensions: Option<ClientExtensions>,
}
