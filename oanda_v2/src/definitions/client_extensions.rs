use crate::definitions::client_comment::ClientComment;
use crate::definitions::client_id::ClientID;
use crate::definitions::client_tag::ClientTag;
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
        Self {
            id: Default::default(),
            tag: Default::default(),
            comment: Default::default(),
        }
    }
}
