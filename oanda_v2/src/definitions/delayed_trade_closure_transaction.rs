use definitions::transaction_id::TransactionID;
use chrono::DateTime;
use definitions::account_id::AccountID;
use definitions::transaction_type::TransactionType;
use definitions::request_id::RequestID;
use definitions::market_order_reason::MarketOrderReason;
use definitions::trade_id::TradeID;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct DelayedTradeClosureTransaction {
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
    /// set to “DELAYED_TRADE_CLOSURE” for an
    /// DelayedTradeClosureTransaction.
    #[serde_inline_default("DELAYED_TRADE_CLOSURE")]
    r#type: TransactionType,
    /// The reason for the delayed trade closure
    reason: Option<MarketOrderReason>,
    /// List of Trade ID’s identifying the open trades that will be
    /// closed when their respective instruments become tradeable
    trade_i_ds: Option<TradeID>,
}
impl Default for DelayedTradeClosureTransaction {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            time: default(),
            user_id: default(),
            account_id: default(),
            batch_id: default(),
            request_id: default(),
            r#type: "DELAYED_TRADE_CLOSURE",
            reason: default(),
            trade_i_ds: default(),
        }
    }
}
