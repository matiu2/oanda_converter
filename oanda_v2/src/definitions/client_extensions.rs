use definitions::client_tag::ClientTag;
use definitions::client_comment::ClientComment;
use definitions::client_id::ClientID;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct ClientExtensions {
    /// The Client ID of the Order/Trade
    id: Option<ClientID>,
    /// A tag associated with the Order/Trade
    tag: Option<ClientTag>,
    /// A comment associated with the Order/Trade
    comment: Option<ClientComment>,
}
impl Default for ClientExtensions {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            tag: default(),
            comment: default(),
        }
    }
}
