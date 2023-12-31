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
