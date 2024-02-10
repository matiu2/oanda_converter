use serde_inline_default::serde_inline_default;
use crate::definitions::account_properties::AccountProperties;
/// The list of authorized Accounts has been provided.
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct Accounts {
    /// The list of Accounts the client is authorized to access and
    /// their associated properties.
    accounts: Vec<AccountProperties>,
}
impl Default for Accounts {
    fn default() -> Self {
        Self {
            accounts: Default::default(),
        }
    }
}
