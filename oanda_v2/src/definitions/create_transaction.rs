use definitions::transaction_id::TransactionID;
use definitions::transaction_type::TransactionType;
use chrono::DateTime;
use definitions::account_id::AccountID;
use definitions::request_id::RequestID;
use definitions::currency::Currency;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct CreateTransaction {
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
    /// The Type of the Transaction. Always set to “CREATE” in a
    /// CreateTransaction.
    #[serde_inline_default("CREATE")]
    r#type: TransactionType,
    /// The ID of the Division that the Account is in
    division_id: Option<integer>,
    /// The ID of the Site that the Account was created at
    site_id: Option<integer>,
    /// The ID of the user that the Account was created for
    account_user_id: Option<integer>,
    /// The number of the Account within the site/division/user
    account_number: Option<integer>,
    /// The home currency of the Account
    home_currency: Option<Currency>,
}
impl Default for CreateTransaction {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            time: default(),
            user_id: default(),
            account_id: default(),
            batch_id: default(),
            request_id: default(),
            r#type: "CREATE",
            division_id: default(),
            site_id: default(),
            account_user_id: default(),
            account_number: default(),
            home_currency: default(),
        }
    }
}
