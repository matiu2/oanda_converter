use definitions::client_configure_transaction::ClientConfigureTransaction;
use definitions::transaction_id::TransactionID;
use endpoints::account::responses::configuration::Configuration400;
use definitions::client_configure_reject_transaction::ClientConfigureRejectTransaction;
use endpoints::account::responses::configuration::Configuration403;
use serde::{Serialize, Deserialize};
/// The Account was configured successfully.
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Configuration200 {
    /// The transaction that configures the Account.
    client_configure_transaction: Option<ClientConfigureTransaction>,
    /// The ID of the last Transaction created for the Account.
    last_transaction_id: Option<TransactionID>,
}
impl Default for Configuration200 {
    fn default() -> Self {
        use Default::default;
        Self {
            client_configure_transaction: default(),
            last_transaction_id: default(),
        }
    }
}
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Configuration400 {
    /// The transaction that rejects the configuration of the
    /// Account.
    client_configure_reject_transaction: Option<ClientConfigureRejectTransaction>,
    /// The ID of the last Transaction created for the Account.
    last_transaction_id: Option<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<String>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: String,
}
impl Default for Configuration400 {
    fn default() -> Self {
        use Default::default;
        Self {
            client_configure_reject_transaction: default(),
            last_transaction_id: default(),
            error_code: default(),
            error_message: default(),
        }
    }
}
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Configuration403 {
    /// The transaction that rejects the configuration of the
    /// Account.
    client_configure_reject_transaction: Option<ClientConfigureRejectTransaction>,
    /// The ID of the last Transaction created for the Account.
    last_transaction_id: Option<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<String>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: String,
}
impl Default for Configuration403 {
    fn default() -> Self {
        use Default::default;
        Self {
            client_configure_reject_transaction: default(),
            last_transaction_id: default(),
            error_code: default(),
            error_message: default(),
        }
    }
}
#[derive(Debug)]
pub enum Error {
    E400(Configuration400),
    E403(Configuration403),
}
