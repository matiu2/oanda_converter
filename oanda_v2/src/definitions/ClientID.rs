/// A client-provided identifier, used by clients to refer to their Orders or Trades with an identifier that they have provided.
///
/// my_order_id
struct client_id(String);
impl std::ops::Deref for client_id {
    type Target = &str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
