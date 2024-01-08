use serde::{Serialize, Deserialize};
/// Currency name identifier. Used by clients to refer to
/// currencies.
///
/// A string containing an ISO 4217 currency (
struct Currency(String);
impl std::ops::Deref for Currency {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
