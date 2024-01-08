use serde::{Serialize, Deserialize};
/// A client-provided identifier, used by clients to refer to
/// their Orders or Trades with an identifier that they have
/// provided.
///
/// my_order_id
struct ClientID(String);
impl std::ops::Deref for ClientID {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
