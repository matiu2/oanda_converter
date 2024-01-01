use definitions::trade_id::TradeID;
use definitions::client_id::ClientID;
use chrono::DateTime;
use definitions::transaction_type::TransactionType;
use definitions::client_extensions::ClientExtensions;
use definitions::account_id::AccountID;
use definitions::request_id::RequestID;
use definitions::transaction_id::TransactionID;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct TradeClientExtensionsModifyTransaction {
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
    /// The Type of the Transaction. Always set
    /// to “TRADE_CLIENT_EXTENSIONS_MODIFY” for a
    /// TradeClientExtensionsModifyTransaction.
    #[serde_inline_default("TRADE_CLIENT_EXTENSIONS_MODIFY")]
    r#type: TransactionType,
    /// The ID of the Trade who’s client extensions are to be
    /// modified.
    trade_id: Option<TradeID>,
    /// The original Client ID of the Trade who’s client extensions
    /// are to be modified.
    client_trade_id: Option<ClientID>,
    /// The new Client Extensions for the Trade.
    trade_client_extensions_modify: Option<ClientExtensions>,
}
impl Default for TradeClientExtensionsModifyTransaction {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            time: default(),
            user_id: default(),
            account_id: default(),
            batch_id: default(),
            request_id: default(),
            r#type: "TRADE_CLIENT_EXTENSIONS_MODIFY",
            trade_id: default(),
            client_trade_id: default(),
            trade_client_extensions_modify: default(),
        }
    }
}
