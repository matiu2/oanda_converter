use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct PositionSide {
    /// Number of units in the position (negative value indicates short position, positive indicates long position).
    units: Option<DecimalNumber>,
    /// Volume-weighted average of the underlying Trade open prices for the Position.
    averagePrice: Option<PriceValue>,
    /// List of the open Trade IDs which contribute to the open Position.
    tradeIDs: Vec<TradeID>,
    /// Profit/loss realized by the PositionSide over the lifetime of the Account.
    pl: Option<AccountUnits>,
    /// The unrealized profit/loss of all open Trades that contribute to this PositionSide.
    unrealizedPL: Option<AccountUnits>,
    /// Profit/loss realized by the PositionSide since the Account’s resettablePL was last reset by the client.
    resettablePL: Option<AccountUnits>,
    /// The total amount of financing paid/collected for this PositionSide over the lifetime of the Account.
    financing: Option<AccountUnits>,
    /// The total amount of dividend adjustment paid for the PositionSide over the lifetime of the Account.
    dividendAdjustment: Option<AccountUnits>,
    /// The total amount of fees charged over the lifetime of the Account for the execution of guaranteed Stop Loss Orders attached to Trades for this PositionSide.
    guaranteedExecutionFees: Option<AccountUnits>,
}
