/// The request identifier.
///
struct RequestID(String);
_blank_!();
impl std::ops::Deref for RequestID {
    type Target = &str;
    _blank_!();
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
