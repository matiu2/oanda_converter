use definitions::transaction_type::TransactionType;
use chrono::DateTime;
use definitions::request_id::RequestID;
use definitions::account_units::AccountUnits;
use definitions::transaction_id::TransactionID;
use definitions::account_id::AccountID;
use definitions::funding_reason::FundingReason;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct TransferFundsTransaction {
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
    /// The Type of the Transaction. Always set to “TRANSFER_FUNDS”
    /// in a TransferFundsTransaction.
    #[serde_inline_default("TRANSFER_FUNDS")]
    r#type: TransactionType,
    /// The amount to deposit/withdraw from the Account in the
    /// Account’s home currency. A positive value indicates a
    /// deposit, a negative value indicates a withdrawal.
    amount: Option<AccountUnits>,
    /// The reason that an Account is being funded.
    funding_reason: Option<FundingReason>,
    /// An optional comment that may be attached to a fund transfer
    /// for audit purposes
    comment: Option<String>,
    /// The Account’s balance after funds are transferred.
    account_balance: Option<AccountUnits>,
}
impl Default for TransferFundsTransaction {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            time: default(),
            user_id: default(),
            account_id: default(),
            batch_id: default(),
            request_id: default(),
            r#type: "TRANSFER_FUNDS",
            amount: default(),
            funding_reason: default(),
            comment: default(),
            account_balance: default(),
        }
    }
}
