/// The request identifier.
///
struct request_id(String);
impl std::ops::Deref for request_id {
    type Target = &str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
