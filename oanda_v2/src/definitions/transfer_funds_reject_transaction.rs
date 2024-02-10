use crate::definitions::transaction_reject_reason::TransactionRejectReason;
use chrono::Utc;
use crate::definitions::transaction_id::TransactionID;
use crate::definitions::request_id::RequestID;
use chrono::DateTime;
use crate::definitions::account_id::AccountID;
use crate::definitions::transaction_type::TransactionType;
use crate::definitions::account_units::AccountUnits;
use crate::definitions::funding_reason::FundingReason;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct TransferFundsRejectTransaction {
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
    /// The Type of the Transaction. Always set to
    /// “TRANSFER_FUNDS_REJECT” in a TransferFundsRejectTransaction.
    #[serde_inline_default("TRANSFER_FUNDS_REJECT")]
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
    /// The reason that the Reject Transaction was created
    reject_reason: Option<TransactionRejectReason>,
}
impl Default for TransferFundsRejectTransaction {
    fn default() -> Self {
        Self {
            id: Default::default(),
            time: Default::default(),
            user_id: Default::default(),
            account_id: Default::default(),
            batch_id: Default::default(),
            request_id: Default::default(),
            r#type: "TRANSFER_FUNDS_REJECT",
            amount: Default::default(),
            funding_reason: Default::default(),
            comment: Default::default(),
            reject_reason: Default::default(),
        }
    }
}
