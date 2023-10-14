/// A client provided request identifier.
///
/// my_request_id
struct ClientRequestID(String);
impl std::ops::Deref for ClientRequestID {
    type Target = &str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
