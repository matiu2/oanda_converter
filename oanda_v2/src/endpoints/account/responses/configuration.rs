use crate::definitions::client_configure_transaction::ClientConfigureTransaction;
use crate::definitions::transaction_id::TransactionID;
use crate::definitions::client_configure_reject_transaction::ClientConfigureRejectTransaction;
/// The Account was configured successfully.
#[derive(Serialize, Deserialize)]
pub struct Configuration200 {
    /// The transaction that configures the Account.
    client_configure_transaction: Option<ClientConfigureTransaction>,
    /// The ID of the last Transaction created for the Account.
    last_transaction_id: Option<TransactionID>,
}
impl Default for Configuration200 {
    fn default() -> Self {
        Self {
            client_configure_transaction: Default::default(),
            last_transaction_id: Default::default(),
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
        Self {
            client_configure_reject_transaction: Default::default(),
            last_transaction_id: Default::default(),
            error_code: Default::default(),
            error_message: Default::default(),
        }
    }
}
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
        Self {
            client_configure_reject_transaction: Default::default(),
            last_transaction_id: Default::default(),
            error_code: Default::default(),
            error_message: Default::default(),
        }
    }
}
#[derive(Debug)]
pub enum Error {
    E400(Configuration400),
    E403(Configuration403),
}
