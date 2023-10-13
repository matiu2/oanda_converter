use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct TradeClientExtensionsModifyRejectTransaction {
    /// The Transaction’s Identifier.
    id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    time: Option<DateTime>,
    /// The ID of the user that initiated the creation of the Transaction.
    user_id: Option<integer>,
    /// The ID of the Account the Transaction was created for.
    account_id: Option<AccountID>,
    /// The ID of the “batch” that the Transaction belongs to. Transactions in the same batch are applied to the Account simultaneously.
    batch_id: Option<TransactionID>,
    /// The Request ID of the request which generated the transaction.
    request_id: Option<RequestID>,
    /// The Type of the Transaction. Always set to “TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT” for a TradeClientExtensionsModifyRejectTransaction.
    #[serde(default = "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT")]
    r#type: TransactionType,
    /// The ID of the Trade who’s client extensions are to be modified.
    trade_id: Option<TradeID>,
    /// The original Client ID of the Trade who’s client extensions are to be modified.
    client_trade_id: Option<ClientID>,
    /// The new Client Extensions for the Trade.
    trade_client_extensions_modify: Option<ClientExtensions>,
    /// The reason that the Reject Transaction was created
    reject_reason: Option<TransactionRejectReason>,
}
