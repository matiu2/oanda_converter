use chrono::DateTime;
use definitions::order_id::OrderID;
use definitions::client_id::ClientID;
use definitions::transaction_reject_reason::TransactionRejectReason;
use definitions::transaction_id::TransactionID;
use definitions::request_id::RequestID;
use definitions::account_id::AccountID;
use definitions::client_extensions::ClientExtensions;
use definitions::transaction_type::TransactionType;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct OrderClientExtensionsModifyRejectTransaction {
    /// The Transaction’s Identifier.
    id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    time: Option<DateTime>,
    /// The ID of the user that initiated the creation of the
    /// Transaction.
    user_id: Option<integer>,
    /// The ID of the Account the Transaction was created for.
    account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to.
    /// Transactions in the same batch are applied to the Account
    /// simultaneously.
    batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the
    /// transaction.
    request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to
    /// “ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT” for a
    /// OrderClientExtensionsModifyRejectTransaction.
    #[serde_inline_default("ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT")]
    r#type: TransactionType,
    /// The ID of the Order who’s client extensions are to be
    /// modified.
    order_id: Option<OrderID>,
    /// The original Client ID of the Order who’s client extensions
    /// are to be modified.
    client_order_id: Option<ClientID>,
    /// The new Client Extensions for the Order.
    client_extensions_modify: Option<ClientExtensions>,
    /// The new Client Extensions for the Order’s Trade on fill.
    trade_client_extensions_modify: Option<ClientExtensions>,
    /// The reason that the Reject Transaction was created
    reject_reason: Option<TransactionRejectReason>,
}
impl Default for OrderClientExtensionsModifyRejectTransaction {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            time: default(),
            user_id: default(),
            account_id: default(),
            batch_id: default(),
            request_id: default(),
            r#type: "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT",
            order_id: default(),
            client_order_id: default(),
            client_extensions_modify: default(),
            trade_client_extensions_modify: default(),
            reject_reason: default(),
        }
    }
}
