use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct OrderIdentifier {
    /// The OANDA-assigned Order ID
    order_id: Option<OrderID>,
    /// The client-provided client Order ID
    client_order_id: Option<ClientID>,
}
