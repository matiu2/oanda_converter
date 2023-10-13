/// The specification of an Order as referred to by clients
///
/// Format: Either the Order’s OANDA-assigned OrderID or the Order’s client-provided ClientID prefixed by the “@” symbol
///
/// Example: 1523
struct order_specifier(String);
impl std::ops::Deref for order_specifier {
    type Target = &str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
