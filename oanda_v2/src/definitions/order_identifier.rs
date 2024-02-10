use crate::definitions::order_id::OrderID;
use crate::definitions::client_id::ClientID;
use serde_inline_default::serde_inline_default;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct OrderIdentifier {
    /// The OANDA-assigned Order ID
    order_id: Option<OrderID>,
    /// The client-provided client Order ID
    client_order_id: Option<ClientID>,
}
impl Default for OrderIdentifier {
    fn default() -> Self {
        Self {
            order_id: Default::default(),
            client_order_id: Default::default(),
        }
    }
}
