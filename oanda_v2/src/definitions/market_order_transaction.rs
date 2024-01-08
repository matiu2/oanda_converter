use crate::definitions::time_in_force::TimeInForce;
use crate::definitions::price_value::PriceValue;
use crate::definitions::instrument_name::InstrumentName;
use crate::definitions::market_order_reason::MarketOrderReason;
use crate::definitions::take_profit_details::TakeProfitDetails;
use crate::definitions::trailing_stop_loss_details::TrailingStopLossDetails;
use crate::definitions::guaranteed_stop_loss_details::GuaranteedStopLossDetails;
use chrono::DateTime;
use crate::definitions::transaction_id::TransactionID;
use crate::definitions::market_order_delayed_trade_close::MarketOrderDelayedTradeClose;
use crate::definitions::transaction_type::TransactionType;
use crate::definitions::request_id::RequestID;
use crate::definitions::decimal_number::DecimalNumber;
use crate::definitions::account_id::AccountID;
use crate::definitions::market_order_trade_close::MarketOrderTradeClose;
use crate::definitions::client_extensions::ClientExtensions;
use crate::definitions::stop_loss_details::StopLossDetails;
use crate::definitions::market_order_margin_closeout::MarketOrderMarginCloseout;
use crate::definitions::order_position_fill::OrderPositionFill;
use crate::definitions::market_order_position_closeout::MarketOrderPositionCloseout;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct MarketOrderTransaction {
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
    /// The Type of the Transaction. Always set to “MARKET_ORDER” in
    /// a MarketOrderTransaction.
    #[serde_inline_default("MARKET_ORDER")]
    r#type: TransactionType,
    /// The Market Order’s Instrument.
    instrument: InstrumentName,
    /// The quantity requested to be filled by the Market Order.
    /// A positive number of units results in a long Order, and a
    /// negative number of units results in a short Order.
    units: DecimalNumber,
    /// The time-in-force requested for the Market Order. Restricted
    /// to FOK or IOC for a MarketOrder.
    #[serde_inline_default("FOK")]
    time_in_force: TimeInForce,
    /// The worst price that the client is willing to have the
    /// Market Order filled at.
    price_bound: Option<PriceValue>,
    /// Specification of how Positions in the Account are modified
    /// when the Order is filled.
    #[serde_inline_default("DEFAULT")]
    position_fill: OrderPositionFill,
    /// Details of the Trade requested to be closed, only provided
    /// when the Market Order is being used to explicitly close
    /// a Trade.
    trade_close: Option<MarketOrderTradeClose>,
    /// Details of the long Position requested to be closed
    /// out, only provided when a Market Order is being used to
    /// explicitly closeout a long Position.
    long_position_closeout: Option<MarketOrderPositionCloseout>,
    /// Details of the short Position requested to be closed
    /// out, only provided when a Market Order is being used to
    /// explicitly closeout a short Position.
    short_position_closeout: Option<MarketOrderPositionCloseout>,
    /// Details of the Margin Closeout that this Market Order was
    /// created for
    margin_closeout: Option<MarketOrderMarginCloseout>,
    /// Details of the delayed Trade close that this Market Order
    /// was created for
    delayed_trade_close: Option<MarketOrderDelayedTradeClose>,
    /// The reason that the Market Order was created
    reason: Option<MarketOrderReason>,
    /// Client Extensions to add to the Order (only provided if the
    /// Order is being created with client extensions).
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
impl Default for MarketOrderTransaction {
    fn default() -> Self {
        Self {
            id: Default::default(),
            time: Default::default(),
            user_id: Default::default(),
            account_id: Default::default(),
            batch_id: Default::default(),
            request_id: Default::default(),
            r#type: "MARKET_ORDER",
            instrument: Default::default(),
            units: Default::default(),
            time_in_force: "FOK",
            price_bound: Default::default(),
            position_fill: "DEFAULT",
            trade_close: Default::default(),
            long_position_closeout: Default::default(),
            short_position_closeout: Default::default(),
            margin_closeout: Default::default(),
            delayed_trade_close: Default::default(),
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
