use chrono::DateTime;
use definitions::transaction_type::TransactionType;
use definitions::request_id::RequestID;
use definitions::account_units::AccountUnits;
use definitions::transaction_id::TransactionID;
use definitions::account_id::AccountID;
use definitions::position_financing::PositionFinancing;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct DailyFinancingTransaction {
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
    /// The Type of the Transaction. Always set to “DAILY_FINANCING”
    /// for a DailyFinancingTransaction.
    #[serde_inline_default("DAILY_FINANCING")]
    r#type: TransactionType,
    /// The amount of financing paid/collected for the Account.
    financing: Option<AccountUnits>,
    /// The Account’s balance after daily financing.
    account_balance: Option<AccountUnits>,
    /// The financing paid/collected for each Position in the
    /// Account.
    position_financings: Vec<PositionFinancing>,
}
impl Default for DailyFinancingTransaction {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            time: default(),
            user_id: default(),
            account_id: default(),
            batch_id: default(),
            request_id: default(),
            r#type: "DAILY_FINANCING",
            financing: default(),
            account_balance: default(),
            position_financings: default(),
        }
    }
}
