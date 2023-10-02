/// A client provided request identifier.
///
/// my_request_id
struct ClientRequestID(String);
_blank_!();
impl std::ops::Deref for ClientRequestID {
    type Target = &str;
    _blank_!();
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
