/// A client-provided tag that can contain any data and may be assigned to their Orders or Trades. Tags are typically used to associate groups of Trades and/or Orders together.
///
/// client_tag_1
struct client_tag(String);
impl std::ops::Deref for client_tag {
    type Target = &str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
