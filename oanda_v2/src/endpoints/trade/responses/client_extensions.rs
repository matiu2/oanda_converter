use definitions::trade_client_extensions_modify_reject_transaction::TradeClientExtensionsModifyRejectTransaction;
use endpoints::trade::responses::client_extensions::ClientExtensions404;
use endpoints::trade::responses::client_extensions::ClientExtensions400;
use definitions::trade_client_extensions_modify_transaction::TradeClientExtensionsModifyTransaction;
use definitions::transaction_id::TransactionID;
use serde::{Serialize, Deserialize};
/// The Trade’s Client Extensions have been updated as
/// requested.
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct ClientExtensions200 {
    /// The Transaction that updates the Trade’s Client Extensions.
    trade_client_extensions_modify_transaction: Option<
        TradeClientExtensionsModifyTransaction,
    >,
    /// The IDs of all Transactions that were created while
    /// satisfying the request.
    related_transaction_i_ds: Vec<TransactionID>,
    /// The ID of the most recent Transaction created for the
    /// Account
    last_transaction_id: Option<TransactionID>,
}
impl Default for ClientExtensions200 {
    fn default() -> Self {
        use Default::default;
        Self {
            trade_client_extensions_modify_transaction: default(),
            related_transaction_i_ds: default(),
            last_transaction_id: default(),
        }
    }
}
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct ClientExtensions400 {
    /// The Transaction that rejects the modification of the Trade’s
    /// Client Extensions.
    trade_client_extensions_modify_reject_transaction: Option<
        TradeClientExtensionsModifyRejectTransaction,
    >,
    /// The ID of the most recent Transaction created for the
    /// Account.
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
            trade_client_extensions_modify_reject_transaction: default(),
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
    /// The Transaction that rejects the modification of the Trade’s
    /// Client Extensions. Only present if the Account exists.
    trade_client_extensions_modify_reject_transaction: Option<
        TradeClientExtensionsModifyRejectTransaction,
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
            trade_client_extensions_modify_reject_transaction: default(),
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
