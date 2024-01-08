use serde::{Serialize, Deserialize};
/// A client provided request identifier.
///
/// my_request_id
pub struct ClientRequestID(String);
impl std::ops::Deref for ClientRequestID {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
