use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct ClientExtensions {
    /// The Client ID of the Order/Trade
    id: Option<ClientID>,
    /// A tag associated with the Order/Trade
    tag: Option<ClientTag>,
    /// A comment associated with the Order/Trade
    comment: Option<ClientComment>,
}
