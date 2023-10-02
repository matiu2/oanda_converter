/// A client-provided identifier, used by clients to refer to their Orders or Trades with an identifier that they have provided.
///
/// my_order_id
struct ClientID(String);
_blank_!();
impl std::ops::Deref for ClientID {
    type Target = &str;
    _blank_!();
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
