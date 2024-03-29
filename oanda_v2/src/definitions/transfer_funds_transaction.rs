use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[serde_inline_default]
pub struct TransferFundsTransaction {
    /// The Transaction’s Identifier.
    id: Option<TransactionID>,
    /// The date/time when the Transaction was created.
    time: Option<DateTime<Utc>>,
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
        Self {
            id: Default::default(),
            time: Default::default(),
            user_id: Default::default(),
            account_id: Default::default(),
            batch_id: Default::default(),
            request_id: Default::default(),
            r#type: "TRANSFER_FUNDS",
            amount: Default::default(),
            funding_reason: Default::default(),
            comment: Default::default(),
            account_balance: Default::default(),
        }
    }
}
