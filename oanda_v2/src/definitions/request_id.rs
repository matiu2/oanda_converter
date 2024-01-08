use serde::{Serialize, Deserialize};
/// The request identifier.
///
///
pub struct RequestID(String);
impl std::ops::Deref for RequestID {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
