use definitions::transaction_id::TransactionID;
use definitions::order_id::OrderID;
use definitions::instrument_name::InstrumentName;
use definitions::account_units::AccountUnits;
use definitions::trade_state::TradeState;
use definitions::client_extensions::ClientExtensions;
use definitions::decimal_number::DecimalNumber;
use definitions::trade_id::TradeID;
use definitions::price_value::PriceValue;
use chrono::DateTime;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct TradeSummary {
    /// The Trade’s identifier, unique within the Trade’s Account.
    id: Option<TradeID>,
    /// The Trade’s Instrument.
    instrument: Option<InstrumentName>,
    /// The execution price of the Trade.
    price: Option<PriceValue>,
    /// The date/time when the Trade was opened.
    open_time: Option<DateTime>,
    /// The current state of the Trade.
    state: Option<TradeState>,
    /// The initial size of the Trade. Negative values indicate a
    /// short Trade, and positive values indicate a long Trade.
    initial_units: Option<DecimalNumber>,
    /// The margin required at the time the Trade was created.
    /// Note, this is the ‘pure’ margin required, it is not the
    /// ‘effective’ margin used that factors in the trade risk if a
    /// GSLO is attached to the trade.
    initial_margin_required: Option<AccountUnits>,
    /// The number of units currently open for the Trade. This value
    /// is reduced to 0.0 as the Trade is closed.
    current_units: Option<DecimalNumber>,
    /// The total profit/loss realized on the closed portion of
    /// the Trade.
    realized_pl: Option<AccountUnits>,
    /// The unrealized profit/loss on the open portion of the Trade.
    unrealized_pl: Option<AccountUnits>,
    /// Margin currently used by the Trade.
    margin_used: Option<AccountUnits>,
    /// The average closing price of the Trade. Only present if the
    /// Trade has been closed or reduced at least once.
    average_close_price: Option<PriceValue>,
    /// The IDs of the Transactions that have closed portions of
    /// this Trade.
    closing_transaction_i_ds: Vec<TransactionID>,
    /// The financing paid/collected for this Trade.
    financing: Option<AccountUnits>,
    /// The dividend adjustment paid for this Trade.
    dividend_adjustment: Option<AccountUnits>,
    /// The date/time when the Trade was fully closed. Only provided
    /// for Trades whose state is CLOSED.
    close_time: Option<DateTime>,
    /// The client extensions of the Trade.
    client_extensions: Option<ClientExtensions>,
    /// ID of the Trade’s Take Profit Order, only provided if such
    /// an Order exists.
    take_profit_order_id: Option<OrderID>,
    /// ID of the Trade’s Stop Loss Order, only provided if such an
    /// Order exists.
    stop_loss_order_id: Option<OrderID>,
    /// ID of the Trade’s Guaranteed Stop Loss Order, only provided
    /// if such an Order exists.
    guaranteed_stop_loss_order_id: Option<OrderID>,
    /// ID of the Trade’s Trailing Stop Loss Order, only provided if
    /// such an Order exists.
    trailing_stop_loss_order_id: Option<OrderID>,
}
impl Default for TradeSummary {
    fn default() -> Self {
        use Default::default;
        Self {
            id: default(),
            instrument: default(),
            price: default(),
            open_time: default(),
            state: default(),
            initial_units: default(),
            initial_margin_required: default(),
            current_units: default(),
            realized_pl: default(),
            unrealized_pl: default(),
            margin_used: default(),
            average_close_price: default(),
            closing_transaction_i_ds: default(),
            financing: default(),
            dividend_adjustment: default(),
            close_time: default(),
            client_extensions: default(),
            take_profit_order_id: default(),
            stop_loss_order_id: default(),
            guaranteed_stop_loss_order_id: default(),
            trailing_stop_loss_order_id: default(),
        }
    }
}
