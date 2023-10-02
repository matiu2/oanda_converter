/// A client-provided comment that can contain any data and may be assigned to their Orders or Trades. Comments are typically used to provide extra context or meaning to an Order or Trade.
///
/// This is a client comment
struct ClientComment(String);
_blank_!();
impl std::ops::Deref for ClientComment {
    type Target = &str;
    _blank_!();
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
