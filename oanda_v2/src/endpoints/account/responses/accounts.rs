use serde::{Serialize, Deserialize};
/// The list of authorized Accounts has been provided.
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Accounts200 {
    /// The list of Accounts the client is authorized to access and
    /// their associated properties.
    accounts: Vec<AccountProperties>,
}
