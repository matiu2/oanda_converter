use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct AccountProperties {
    /// The Account’s identifier
    id: Option<AccountID>,
    /// The Account’s associated MT4 Account ID. This field will not
    /// be present if the Account is not an MT4 account.
    mt4_account_id: Option<integer>,
    /// The Account’s tags
    tags: Vec<String>,
}
impl Default for AccountProperties {
    fn default() -> Self {
        Self {
            id: Default::default(),
            mt4_account_id: Default::default(),
            tags: Default::default(),
        }
    }
}
