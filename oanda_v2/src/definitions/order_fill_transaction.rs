use crate::definitions::order_fill_reason::OrderFillReason;
use crate::definitions::trade_reduce::TradeReduce;
use crate::definitions::order_id::OrderID;
use crate::definitions::price_value::PriceValue;
use crate::definitions::client_id::ClientID;
use crate::definitions::home_conversion_factors::HomeConversionFactors;
use crate::definitions::transaction_id::TransactionID;
use chrono::DateTime;
use crate::definitions::trade_open::TradeOpen;
use crate::definitions::client_price::ClientPrice;
use crate::definitions::account_id::AccountID;
use crate::definitions::transaction_type::TransactionType;
use crate::definitions::request_id::RequestID;
use crate::definitions::decimal_number::DecimalNumber;
use crate::definitions::instrument_name::InstrumentName;
use crate::definitions::account_units::AccountUnits;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct OrderFillTransaction {
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
    /// The Type of the Transaction. Always set to “ORDER_FILL” for
    /// an OrderFillTransaction.
    #[serde_inline_default("ORDER_FILL")]
    r#type: TransactionType,
    /// The ID of the Order filled.
    order_id: Option<OrderID>,
    /// The client Order ID of the Order filled (only provided if
    /// the client has assigned one).
    client_order_id: Option<ClientID>,
    /// The name of the filled Order’s instrument.
    instrument: Option<InstrumentName>,
    /// The number of units filled by the OrderFill.
    units: Option<DecimalNumber>,
    /// The HomeConversionFactors in effect at the time of the
    /// OrderFill.
    home_conversion_factors: Option<HomeConversionFactors>,
    /// The price that all of the units of the OrderFill should
    /// have been filled at, in the absence of guaranteed
    /// price execution. This factors in the Account’s current
    /// ClientPrice, used liquidity and the units of the OrderFill
    /// only. If no Trades were closed with their price clamped for
    /// guaranteed stop loss enforcement, then this value will match
    /// the price fields of each Trade opened, closed, and reduced,
    /// and they will all be the exact same.
    full_vwap: Option<PriceValue>,
    /// The price in effect for the account at the time of the Order
    /// fill.
    full_price: Option<ClientPrice>,
    /// The reason that an Order was filled
    reason: Option<OrderFillReason>,
    /// The profit or loss incurred when the Order was filled.
    pl: Option<AccountUnits>,
    /// The profit or loss incurred when the Order was filled, in
    /// the Instrument’s quote currency.
    quote_pl: Option<DecimalNumber>,
    /// The financing paid or collected when the Order was filled.
    financing: Option<AccountUnits>,
    /// The financing paid or collected when the Order was filled,
    /// in the Instrument’s base currency.
    base_financing: Option<DecimalNumber>,
    /// The financing paid or collected when the Order was filled,
    /// in the Instrument’s quote currency.
    quote_financing: Option<DecimalNumber>,
    /// The commission charged in the Account’s home currency as
    /// a result of filling the Order. The commission is always
    /// represented as a positive quantity of the Account’s home
    /// currency, however it reduces the balance in the Account.
    commission: Option<AccountUnits>,
    /// The total guaranteed execution fees charged for all Trades
    /// opened, closed or reduced with guaranteed Stop Loss Orders.
    guaranteed_execution_fee: Option<AccountUnits>,
    /// The total guaranteed execution fees charged for all Trades
    /// opened, closed or reduced with guaranteed Stop Loss Orders,
    /// expressed in the Instrument’s quote currency.
    quote_guaranteed_execution_fee: Option<DecimalNumber>,
    /// The Account’s balance after the Order was filled.
    account_balance: Option<AccountUnits>,
    /// The Trade that was opened when the Order was filled (only
    /// provided if filling the Order resulted in a new Trade).
    trade_opened: Option<TradeOpen>,
    /// The Trades that were closed when the Order was filled (only
    /// provided if filling the Order resulted in a closing open
    /// Trades).
    trades_closed: Vec<TradeReduce>,
    /// The Trade that was reduced when the Order was filled (only
    /// provided if filling the Order resulted in reducing an open
    /// Trade).
    trade_reduced: Option<TradeReduce>,
    /// The half spread cost for the OrderFill, which is the sum of
    /// the halfSpreadCost values in the tradeOpened, tradesClosed
    /// and tradeReduced fields. This can be a positive or negative
    /// value and is represented in the home currency of the
    /// Account.
    half_spread_cost: Option<AccountUnits>,
}
impl Default for OrderFillTransaction {
    fn default() -> Self {
        Self {
            id: Default::default(),
            time: Default::default(),
            user_id: Default::default(),
            account_id: Default::default(),
            batch_id: Default::default(),
            request_id: Default::default(),
            r#type: "ORDER_FILL",
            order_id: Default::default(),
            client_order_id: Default::default(),
            instrument: Default::default(),
            units: Default::default(),
            home_conversion_factors: Default::default(),
            full_vwap: Default::default(),
            full_price: Default::default(),
            reason: Default::default(),
            pl: Default::default(),
            quote_pl: Default::default(),
            financing: Default::default(),
            base_financing: Default::default(),
            quote_financing: Default::default(),
            commission: Default::default(),
            guaranteed_execution_fee: Default::default(),
            quote_guaranteed_execution_fee: Default::default(),
            account_balance: Default::default(),
            trade_opened: Default::default(),
            trades_closed: Default::default(),
            trade_reduced: Default::default(),
            half_spread_cost: Default::default(),
        }
    }
}
