/// A client provided request identifier.
///
/// my_request_id
struct client_request_id(String);
impl std::ops::Deref for client_request_id {
    type Target = &str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
