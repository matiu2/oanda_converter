use definitions::client_id::ClientID;
use definitions::order_id::OrderID;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct OrderIdentifier {
    /// The OANDA-assigned Order ID
    order_id: Option<OrderID>,
    /// The client-provided client Order ID
    client_order_id: Option<ClientID>,
}
impl Default for OrderIdentifier {
    fn default() -> Self {
        use Default::default;
        Self {
            order_id: default(),
            client_order_id: default(),
        }
    }
}
