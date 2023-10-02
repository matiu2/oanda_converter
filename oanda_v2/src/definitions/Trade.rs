use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct Trade {
    /// The Trade’s identifier, unique within the Trade’s Account.
    id: Option<TradeID>,
    /// The Trade’s Instrument.
    instrument: Option<InstrumentName>,
    /// The execution price of the Trade.
    price: Option<PriceValue>,
    /// The date/time when the Trade was opened.
    openTime: Option<DateTime>,
    /// The current state of the Trade.
    state: Option<TradeState>,
    /// The initial size of the Trade. Negative values indicate a short Trade, and positive values indicate a long Trade.
    initialUnits: Option<DecimalNumber>,
    /// The margin required at the time the Trade was created. Note, this is the ‘pure’ margin required, it is not the ‘effective’ margin used that factors in the trade risk if a GSLO is attached to the trade.
    initialMarginRequired: Option<AccountUnits>,
    /// The number of units currently open for the Trade. This value is reduced to 0.0 as the Trade is closed.
    currentUnits: Option<DecimalNumber>,
    /// The total profit/loss realized on the closed portion of the Trade.
    realizedPL: Option<AccountUnits>,
    /// The unrealized profit/loss on the open portion of the Trade.
    unrealizedPL: Option<AccountUnits>,
    /// Margin currently used by the Trade.
    marginUsed: Option<AccountUnits>,
    /// The average closing price of the Trade. Only present if the Trade has been closed or reduced at least once.
    averageClosePrice: Option<PriceValue>,
    /// The IDs of the Transactions that have closed portions of this Trade.
    closingTransactionIDs: Vec<TransactionID>,
    /// The financing paid/collected for this Trade.
    financing: Option<AccountUnits>,
    /// The dividend adjustment paid for this Trade.
    dividendAdjustment: Option<AccountUnits>,
    /// The date/time when the Trade was fully closed. Only provided for Trades whose state is CLOSED.
    closeTime: Option<DateTime>,
    /// The client extensions of the Trade.
    clientExtensions: Option<ClientExtensions>,
    /// Full representation of the Trade’s Take Profit Order, only provided if such an Order exists.
    takeProfitOrder: Option<TakeProfitOrder>,
    /// Full representation of the Trade’s Stop Loss Order, only provided if such an Order exists.
    stopLossOrder: Option<StopLossOrder>,
    /// Full representation of the Trade’s Trailing Stop Loss Order, only provided if such an Order exists.
    trailingStopLossOrder: Option<TrailingStopLossOrder>,
}
