use crate::definitions::transaction_type::TransactionType;
use crate::definitions::guaranteed_stop_loss_details::GuaranteedStopLossDetails;
use crate::definitions::fixed_price_order_reason::FixedPriceOrderReason;
use crate::definitions::account_id::AccountID;
use chrono::DateTime;
use crate::definitions::price_value::PriceValue;
use crate::definitions::client_extensions::ClientExtensions;
use crate::definitions::instrument_name::InstrumentName;
use crate::definitions::decimal_number::DecimalNumber;
use crate::definitions::request_id::RequestID;
use crate::definitions::transaction_id::TransactionID;
use crate::definitions::stop_loss_details::StopLossDetails;
use crate::definitions::trailing_stop_loss_details::TrailingStopLossDetails;
use crate::definitions::take_profit_details::TakeProfitDetails;
use crate::definitions::order_position_fill::OrderPositionFill;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct FixedPriceOrderTransaction {
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
    /// “FIXED_PRICE_ORDER” in a FixedPriceOrderTransaction.
    #[serde_inline_default("FIXED_PRICE_ORDER")]
    r#type: TransactionType,
    /// The Fixed Price Order’s Instrument.
    instrument: InstrumentName,
    /// The quantity requested to be filled by the Fixed Price
    /// Order. A positive number of units results in a long Order,
    /// and a negative number of units results in a short Order.
    units: DecimalNumber,
    /// The price specified for the Fixed Price Order. This price
    /// is the exact price that the Fixed Price Order will be filled
    /// at.
    price: PriceValue,
    /// Specification of how Positions in the Account are modified
    /// when the Order is filled.
    #[serde_inline_default("DEFAULT")]
    position_fill: OrderPositionFill,
    /// The state that the trade resulting from the Fixed Price
    /// Order should be set to.
    trade_state: String,
    /// The reason that the Fixed Price Order was created
    reason: Option<FixedPriceOrderReason>,
    /// The client extensions for the Fixed Price Order.
    client_extensions: Option<ClientExtensions>,
    /// The specification of the Take Profit Order that should be
    /// created for a Trade opened when the Order is filled (if such
    /// a Trade is created).
    take_profit_on_fill: Option<TakeProfitDetails>,
    /// The specification of the Stop Loss Order that should be
    /// created for a Trade opened when the Order is filled (if such
    /// a Trade is created).
    stop_loss_on_fill: Option<StopLossDetails>,
    /// The specification of the Trailing Stop Loss Order that
    /// should be created for a Trade that is opened when the Order
    /// is filled (if such a Trade is created).
    trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The specification of the Guaranteed Stop Loss Order that
    /// should be created for a Trade that is opened when the Order
    /// is filled (if such a Trade is created).
    guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order
    /// is filled (if such a Trade is created).  Do not set, modify,
    /// delete tradeClientExtensions if your account is associated
    /// with MT4.
    trade_client_extensions: Option<ClientExtensions>,
}
impl Default for FixedPriceOrderTransaction {
    fn default() -> Self {
        Self {
            id: Default::default(),
            time: Default::default(),
            user_id: Default::default(),
            account_id: Default::default(),
            batch_id: Default::default(),
            request_id: Default::default(),
            r#type: "FIXED_PRICE_ORDER",
            instrument: Default::default(),
            units: Default::default(),
            price: Default::default(),
            position_fill: "DEFAULT",
            trade_state: Default::default(),
            reason: Default::default(),
            client_extensions: Default::default(),
            take_profit_on_fill: Default::default(),
            stop_loss_on_fill: Default::default(),
            trailing_stop_loss_on_fill: Default::default(),
            guaranteed_stop_loss_on_fill: Default::default(),
            trade_client_extensions: Default::default(),
        }
    }
}
