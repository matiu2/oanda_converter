/// The identification of a Trade as referred to by clients
///
/// Format: Either the Trade’s OANDA-assigned TradeID or the Trade’s client-provided ClientID prefixed by the “@” symbol
///
/// Example: @my_trade_id
struct TradeSpecifier(String);
_blank_!();
impl std::ops::Deref for TradeSpecifier {
    type Target = &str;
    _blank_!();
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
