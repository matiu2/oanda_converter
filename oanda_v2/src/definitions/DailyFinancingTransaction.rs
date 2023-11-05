use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct DailyFinancingTransaction {
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
    #[serde(default = "DAILY_FINANCING")]
    r#type: TransactionType,
    /// The amount of financing paid/collected for the Account.
    financing: Option<AccountUnits>,
    /// The Account’s balance after daily financing.
    account_balance: Option<AccountUnits>,
    /// The financing paid/collected for each Position in the
    /// Account.
    position_financings: Vec<PositionFinancing>,
}
