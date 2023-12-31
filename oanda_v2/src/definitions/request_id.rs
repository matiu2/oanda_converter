use serde::{Serialize, Deserialize};
/// The request identifier.
///
///
struct RequestID(String);
impl std::ops::Deref for RequestID {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
