use serde::{Serialize, Deserialize};
/// A client-provided tag that can contain any data and may be
/// assigned to their Orders or Trades. Tags are typically used
/// to associate groups of Trades and/or Orders together.
///
/// client_tag_1
pub struct ClientTag(String);
impl std::ops::Deref for ClientTag {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
