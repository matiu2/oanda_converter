use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
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
