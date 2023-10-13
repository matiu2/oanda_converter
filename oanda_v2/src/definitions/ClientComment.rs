/// A client-provided comment that can contain any data and may be assigned to their Orders or Trades. Comments are typically used to provide extra context or meaning to an Order or Trade.
///
/// This is a client comment
struct client_comment(String);
impl std::ops::Deref for client_comment {
    type Target = &str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
