/// The Order’s identifier, unique within the Order’s Account.
///
/// Format: The string representation of the OANDA-assigned OrderID. OANDA-assigned OrderIDs are positive integers, and are derived from the TransactionID of the Transaction that created the Order.
///
/// Example: 1523
struct order_id(String);
impl std::ops::Deref for order_id {
    type Target = &str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
