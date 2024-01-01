use definitions::instrument_name::InstrumentName;
use definitions::order_state::OrderState;
use definitions::price_value::PriceValue;
use definitions::client_extensions::ClientExtensions;
use definitions::decimal_number::DecimalNumber;
use definitions::trailing_stop_loss_details::TrailingStopLossDetails;
use definitions::trade_id::TradeID;
use definitions::take_profit_details::TakeProfitDetails;
use chrono::DateTime;
use definitions::order_id::OrderID;
use definitions::stop_loss_details::StopLossDetails;
use definitions::guaranteed_stop_loss_details::GuaranteedStopLossDetails;
use definitions::transaction_id::TransactionID;
use definitions::order_type::OrderType;
use definitions::order_position_fill::OrderPositionFill;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct FixedPriceOrder {
    /// The Order’s identifier, unique within the Order’s Account.
    id: Option<OrderID>,
    /// The time when the Order was created.
    create_time: Option<DateTime>,
    /// The current state of the Order.
    state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or
    /// delete clientExtensions if your account is associated with
    /// MT4.
    client_extensions: Option<ClientExtensions>,
    /// The type of the Order. Always set to “FIXED_PRICE” for Fixed
    /// Price Orders.
    #[serde_inline_default("FIXED_PRICE")]
    r#type: OrderType,
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
    /// TakeProfitDetails specifies the details of a Take Profit
    /// Order to be created on behalf of a client. This may happen
    /// when an Order is filled that opens a Trade requiring a Take
    /// Profit, or when a Trade’s dependent Take Profit Order is
    /// modified directly through the Trade.
    take_profit_on_fill: Option<TakeProfitDetails>,
    /// StopLossDetails specifies the details of a Stop Loss Order
    /// to be created on behalf of a client. This may happen when
    /// an Order is filled that opens a Trade requiring a Stop Loss,
    /// or when a Trade’s dependent Stop Loss Order is modified
    /// directly through the Trade.
    stop_loss_on_fill: Option<StopLossDetails>,
    /// GuaranteedStopLossDetails specifies the details of a
    /// Guaranteed Stop Loss Order to be created on behalf of a
    /// client. This may happen when an Order is filled that opens
    /// a Trade requiring a Guaranteed Stop Loss, or when a Trade’s
    /// dependent Guaranteed Stop Loss Order is modified directly
    /// through the Trade.
    guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// TrailingStopLossDetails specifies the details of a Trailing
    /// Stop Loss Order to be created on behalf of a client. This
    /// may happen when an Order is filled that opens a Trade
    /// requiring a Trailing Stop Loss, or when a Trade’s dependent
    /// Trailing Stop Loss Order is modified directly through the
    /// Trade.
    trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// Client Extensions to add to the Trade created when the
    /// Order is filled (if such a Trade is created). Do not set,
    /// modify, or delete tradeClientExtensions if your account is
    /// associated with MT4.
    trade_client_extensions: Option<ClientExtensions>,
    /// ID of the Transaction that filled this Order (only provided
    /// when the Order’s state is FILLED)
    filling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was filled (only provided when the
    /// Order’s state is FILLED)
    filled_time: Option<DateTime>,
    /// Trade ID of Trade opened when the Order was filled (only
    /// provided when the Order’s state is FILLED and a Trade was
    /// opened as a result of the fill)
    trade_opened_id: Option<TradeID>,
    /// Trade ID of Trade reduced when the Order was filled (only
    /// provided when the Order’s state is FILLED and a Trade was
    /// reduced as a result of the fill)
    trade_reduced_id: Option<TradeID>,
    /// Trade IDs of Trades closed when the Order was filled (only
    /// provided when the Order’s state is FILLED and one or more
    /// Trades were closed as a result of the fill)
    trade_closed_i_ds: Vec<TradeID>,
    /// ID of the Transaction that cancelled the Order (only
    /// provided when the Order’s state is CANCELLED)
    cancelling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was cancelled (only provided when
    /// the state of the Order is CANCELLED)
    cancelled_time: Option<DateTime>,
}
impl Default for FixedPriceOrder {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            create_time: default(),
            state: default(),
            client_extensions: default(),
            r#type: "FIXED_PRICE",
            instrument: default(),
            units: default(),
            price: default(),
            position_fill: "DEFAULT",
            trade_state: default(),
            take_profit_on_fill: default(),
            stop_loss_on_fill: default(),
            guaranteed_stop_loss_on_fill: default(),
            trailing_stop_loss_on_fill: default(),
            trade_client_extensions: default(),
            filling_transaction_id: default(),
            filled_time: default(),
            trade_opened_id: default(),
            trade_reduced_id: default(),
            trade_closed_i_ds: default(),
            cancelling_transaction_id: default(),
            cancelled_time: default(),
        }
    }
}
