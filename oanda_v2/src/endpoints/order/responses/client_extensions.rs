use endpoints::trade::responses::client_extensions::ClientExtensions404;
use definitions::order_client_extensions_modify_reject_transaction::OrderClientExtensionsModifyRejectTransaction;
use endpoints::trade::responses::client_extensions::ClientExtensions400;
use definitions::order_client_extensions_modify_transaction::OrderClientExtensionsModifyTransaction;
use definitions::transaction_id::TransactionID;
use serde::{Serialize, Deserialize};
/// The Order’s Client Extensions were successfully modified
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct ClientExtensions200 {
    /// The Transaction that modified the Client Extensions for
    /// the Order
    order_client_extensions_modify_transaction: Option<
        OrderClientExtensionsModifyTransaction,
    >,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
}
impl Default for ClientExtensions200 {
    fn default() -> Self {
        use Default::default;
        Self {
            order_client_extensions_modify_transaction: default(),
            last_transaction_id: default(),
            related_transaction_i_ds: default(),
        }
    }
}
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct ClientExtensions400 {
    /// The Transaction that rejected the modification of the Client
    /// Extensions for the Order
    order_client_extensions_modify_reject_transaction: Option<
        OrderClientExtensionsModifyRejectTransaction,
    >,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<String>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: String,
}
impl Default for ClientExtensions400 {
    fn default() -> Self {
        use Default::default;
        Self {
            order_client_extensions_modify_reject_transaction: default(),
            last_transaction_id: default(),
            related_transaction_i_ds: default(),
            error_code: default(),
            error_message: default(),
        }
    }
}
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct ClientExtensions404 {
    /// The Transaction that rejected the modification of the
    /// Client Extensions for the Order. Only present if the Account
    /// exists.
    order_client_extensions_modify_reject_transaction: Option<
        OrderClientExtensionsModifyRejectTransaction,
    >,
    /// The ID of the most recent Transaction created for the
    /// Account. Only present if the Account exists.
    last_transaction_id: Option<TransactionID>,
    /// The IDs of all Transactions that were created while
    /// satisfying the request. Only present if the Account exists.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The code of the error that has occurred. This field may not
    /// be returned for some errors.
    error_code: Option<String>,
    /// The human-readable description of the error that has
    /// occurred.
    error_message: String,
}
impl Default for ClientExtensions404 {
    fn default() -> Self {
        use Default::default;
        Self {
            order_client_extensions_modify_reject_transaction: default(),
            last_transaction_id: default(),
            related_transaction_i_ds: default(),
            error_code: default(),
            error_message: default(),
        }
    }
}
#[derive(Debug)]
pub enum Error {
    E400(ClientExtensions400),
    E404(ClientExtensions404),
}
