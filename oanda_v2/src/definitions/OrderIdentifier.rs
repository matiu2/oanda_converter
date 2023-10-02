use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct OrderIdentifier {
    /// The OANDA-assigned Order ID
    orderID: Option<OrderID>,
    /// The client-provided client Order ID
    clientOrderID: Option<ClientID>,
}
