use crate::definitions::account_properties::AccountProperties;
/// The list of authorized Accounts has been provided.
#[derive(Serialize, Deserialize)]
pub struct Accounts200 {
    /// The list of Accounts the client is authorized to access and
    /// their associated properties.
    accounts: Vec<AccountProperties>,
}
impl Default for Accounts200 {
    fn default() -> Self {
        Self {
            accounts: Default::default(),
        }
    }
}
