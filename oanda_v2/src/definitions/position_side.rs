use definitions::trade_id::TradeID;
use definitions::decimal_number::DecimalNumber;
use definitions::account_units::AccountUnits;
use definitions::price_value::PriceValue;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct PositionSide {
    /// Number of units in the position (negative value indicates
    /// short position, positive indicates long position).
    units: Option<DecimalNumber>,
    /// Volume-weighted average of the underlying Trade open prices
    /// for the Position.
    average_price: Option<PriceValue>,
    /// List of the open Trade IDs which contribute to the open
    /// Position.
    trade_i_ds: Vec<TradeID>,
    /// Profit/loss realized by the PositionSide over the lifetime
    /// of the Account.
    pl: Option<AccountUnits>,
    /// The unrealized profit/loss of all open Trades that
    /// contribute to this PositionSide.
    unrealized_pl: Option<AccountUnits>,
    /// Profit/loss realized by the PositionSide since the Account’s
    /// resettablePL was last reset by the client.
    resettable_pl: Option<AccountUnits>,
    /// The total amount of financing paid/collected for this
    /// PositionSide over the lifetime of the Account.
    financing: Option<AccountUnits>,
    /// The total amount of dividend adjustment paid for the
    /// PositionSide over the lifetime of the Account.
    dividend_adjustment: Option<AccountUnits>,
    /// The total amount of fees charged over the lifetime of the
    /// Account for the execution of guaranteed Stop Loss Orders
    /// attached to Trades for this PositionSide.
    guaranteed_execution_fees: Option<AccountUnits>,
}
