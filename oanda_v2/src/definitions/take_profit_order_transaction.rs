use chrono::DateTime;
use crate::definitions::client_extensions::ClientExtensions;
use crate::definitions::trade_id::TradeID;
use crate::definitions::request_id::RequestID;
use crate::definitions::client_id::ClientID;
use crate::definitions::price_value::PriceValue;
use crate::definitions::account_id::AccountID;
use crate::definitions::take_profit_order_reason::TakeProfitOrderReason;
use crate::definitions::time_in_force::TimeInForce;
use crate::definitions::transaction_type::TransactionType;
use crate::definitions::order_trigger_condition::OrderTriggerCondition;
use crate::definitions::order_id::OrderID;
use crate::definitions::transaction_id::TransactionID;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct TakeProfitOrderTransaction {
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
    /// The Type of the Transaction. Always set to
    /// “TAKE_PROFIT_ORDER” in a TakeProfitOrderTransaction.
    #[serde_inline_default("TAKE_PROFIT_ORDER")]
    r#type: TransactionType,
    /// The ID of the Trade to close when the price threshold is
    /// breached.
    trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price
    /// threshold is breached.
    client_trade_id: Option<ClientID>,
    /// The price threshold specified for the TakeProfit Order. The
    /// associated Trade will be closed by a market price that is
    /// equal to or better than this threshold.
    price: PriceValue,
    /// The time-in-force requested for the TakeProfit Order.
    /// Restricted to “GTC”, “GFD” and “GTD” for TakeProfit Orders.
    #[serde_inline_default("GTC")]
    time_in_force: TimeInForce,
    /// The date/time when the TakeProfit Order will be cancelled if
    /// its timeInForce is “GTD”.
    gtd_time: Option<DateTime>,
    /// Specification of which price component should be used when
    /// determining if an Order should be triggered and filled.
    /// This allows Orders to be triggered based on the bid, ask,
    /// mid, default (ask for buy, bid for sell) or inverse (ask for
    /// sell, bid for buy) price depending on the desired behaviour.
    /// Orders are always filled using their default price
    /// component. This feature is only provided through the REST
    /// API. Clients who choose to specify a non-default trigger
    /// condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction
    /// history or their account statements. OANDA platforms always
    /// assume that an Order’s trigger condition is set to the
    /// default value when indicating the distance from an Order’s
    /// trigger price, and will always provide the default trigger
    /// condition when creating or modifying an Order. A special
    /// restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the TriggerCondition value must either
    /// be “DEFAULT”, or the “natural” trigger side “DEFAULT”
    /// results in. So for a Guaranteed Stop Loss Order for a long
    /// trade valid values are “DEFAULT” and “BID”, and for short
    /// trades “DEFAULT” and “ASK” are valid.
    #[serde_inline_default("DEFAULT")]
    trigger_condition: OrderTriggerCondition,
    /// The reason that the Take Profit Order was initiated
    reason: Option<TakeProfitOrderReason>,
    /// Client Extensions to add to the Order (only provided if the
    /// Order is being created with client extensions).
    client_extensions: Option<ClientExtensions>,
    /// The ID of the OrderFill Transaction that caused this Order
    /// to be created (only provided if this Order was created
    /// automatically when another Order was filled).
    order_fill_transaction_id: Option<TransactionID>,
    /// The ID of the Order that this Order replaces (only provided
    /// if this Order replaces an existing Order).
    replaces_order_id: Option<OrderID>,
    /// The ID of the Transaction that cancels the replaced Order
    /// (only provided if this Order replaces an existing Order).
    cancelling_transaction_id: Option<TransactionID>,
}
impl Default for TakeProfitOrderTransaction {
    fn default() -> Self {
        Self {
            id: Default::default(),
            time: Default::default(),
            user_id: Default::default(),
            account_id: Default::default(),
            batch_id: Default::default(),
            request_id: Default::default(),
            r#type: "TAKE_PROFIT_ORDER",
            trade_id: Default::default(),
            client_trade_id: Default::default(),
            price: Default::default(),
            time_in_force: "GTC",
            gtd_time: Default::default(),
            trigger_condition: "DEFAULT",
            reason: Default::default(),
            client_extensions: Default::default(),
            order_fill_transaction_id: Default::default(),
            replaces_order_id: Default::default(),
            cancelling_transaction_id: Default::default(),
        }
    }
}
