use definitions::request_id::RequestID;
use chrono::DateTime;
use definitions::transaction_id::TransactionID;
use definitions::decimal_number::DecimalNumber;
use definitions::transaction_reject_reason::TransactionRejectReason;
use definitions::transaction_type::TransactionType;
use definitions::account_id::AccountID;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct ClientConfigureRejectTransaction {
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
    /// The Type of the Transaction. Always
    /// set to “CLIENT_CONFIGURE_REJECT” in a
    /// ClientConfigureRejectTransaction.
    #[serde_inline_default("CLIENT_CONFIGURE_REJECT")]
    r#type: TransactionType,
    /// The client-provided alias for the Account.
    alias: Option<String>,
    /// The margin rate override for the Account.
    margin_rate: Option<DecimalNumber>,
    /// The reason that the Reject Transaction was created
    reject_reason: Option<TransactionRejectReason>,
}
impl Default for ClientConfigureRejectTransaction {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            time: default(),
            user_id: default(),
            account_id: default(),
            batch_id: default(),
            request_id: default(),
            r#type: "CLIENT_CONFIGURE_REJECT",
            alias: default(),
            margin_rate: default(),
            reject_reason: default(),
        }
    }
}
