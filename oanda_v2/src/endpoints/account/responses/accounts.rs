use serde::{Serialize, Deserialize};
/// The list of authorized Accounts has been provided.
#[derive(Serialize, Deserialize)]
struct Accounts200 {
    /// The list of Accounts the client is authorized to access and
    /// their associated properties.
    accounts: Vec<AccountProperties>,
}
