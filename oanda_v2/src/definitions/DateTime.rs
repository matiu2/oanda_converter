/// A date and time value using either RFC3339 or UNIX time representation.
///
/// The RFC 3339 representation is a string conforming to
struct DateTime(String);
impl std::ops::Deref for DateTime {
    type Target = &str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
