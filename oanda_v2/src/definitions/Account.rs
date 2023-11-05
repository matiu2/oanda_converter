use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct Account {
    /// The Account’s identifier
    id: Option<AccountID>,
    /// Client-assigned alias for the Account. Only provided if the
    /// Account has an alias set
    alias: Option<string>,
    /// The home currency of the Account
    currency: Option<Currency>,
    /// ID of the user that created the Account.
    created_by_user_id: Option<integer>,
    /// The date/time when the Account was created.
    created_time: Option<DateTime>,
    /// The current guaranteed Stop Loss Order settings of
    /// the Account. This field will only be present if the
    /// guaranteedStopLossOrderMode is not ‘DISABLED’.
    guaranteed_stop_loss_order_parameters: Option<GuaranteedStopLossOrderParameters>,
    /// The current guaranteed Stop Loss Order mode of the Account.
    guaranteed_stop_loss_order_mode: Option<GuaranteedStopLossOrderMode>,
    /// The date/time that the Account’s resettablePL was last
    /// reset.
    resettable_pl_time: Option<DateTime>,
    /// Client-provided margin rate override for the Account. The
    /// effective margin rate of the Account is the lesser of this
    /// value and the OANDA margin rate for the Account’s division.
    /// This value is only provided if a margin rate override exists
    /// for the Account.
    margin_rate: Option<DecimalNumber>,
    /// The number of Trades currently open in the Account.
    open_trade_count: Option<integer>,
    /// The number of Positions currently open in the Account.
    open_position_count: Option<integer>,
    /// The number of Orders currently pending in the Account.
    pending_order_count: Option<integer>,
    /// Flag indicating that the Account has hedging enabled.
    hedging_enabled: Option<boolean>,
    /// The total unrealized profit/loss for all Trades currently
    /// open in the Account.
    unrealized_pl: Option<AccountUnits>,
    /// The net asset value of the Account. Equal to Account balance
    /// + unrealizedPL.
    nav: Option<AccountUnits>,
    /// Margin currently used for the Account.
    margin_used: Option<AccountUnits>,
    /// Margin available for Account currency.
    margin_available: Option<AccountUnits>,
    /// The value of the Account’s open positions represented in the
    /// Account’s home currency.
    position_value: Option<AccountUnits>,
    /// The Account’s margin closeout unrealized PL.
    margin_closeout_unrealized_pl: Option<AccountUnits>,
    /// The Account’s margin closeout NAV.
    margin_closeout_nav: Option<AccountUnits>,
    /// The Account’s margin closeout margin used.
    margin_closeout_margin_used: Option<AccountUnits>,
    /// The Account’s margin closeout percentage. When this value is
    /// 1.0 or above the Account is in a margin closeout situation.
    margin_closeout_percent: Option<DecimalNumber>,
    /// The value of the Account’s open positions as used for margin
    /// closeout calculations represented in the Account’s home
    /// currency.
    margin_closeout_position_value: Option<DecimalNumber>,
    /// The current WithdrawalLimit for the account which will
    /// be zero or a positive value indicating how much can be
    /// withdrawn from the account.
    withdrawal_limit: Option<AccountUnits>,
    /// The Account’s margin call margin used.
    margin_call_margin_used: Option<AccountUnits>,
    /// The Account’s margin call percentage. When this value is 1.0
    /// or above the Account is in a margin call situation.
    margin_call_percent: Option<DecimalNumber>,
    /// The current balance of the account.
    balance: Option<AccountUnits>,
    /// The total profit/loss realized over the lifetime of the
    /// Account.
    pl: Option<AccountUnits>,
    /// The total realized profit/loss for the account since it was
    /// last reset by the client.
    resettable_pl: Option<AccountUnits>,
    /// The total amount of financing paid/collected over the
    /// lifetime of the account.
    financing: Option<AccountUnits>,
    /// The total amount of commission paid over the lifetime of
    /// the Account.
    commission: Option<AccountUnits>,
    /// The total amount of dividend adjustment paid over the
    /// lifetime of the Account in the Account’s home currency.
    dividend_adjustment: Option<AccountUnits>,
    /// The total amount of fees charged over the lifetime of the
    /// Account for the execution of guaranteed Stop Loss Orders.
    guaranteed_execution_fees: Option<AccountUnits>,
    /// The date/time when the Account entered a margin call state.
    /// Only provided if the Account is in a margin call.
    margin_call_enter_time: Option<DateTime>,
    /// The number of times that the Account’s current margin call
    /// was extended.
    margin_call_extension_count: Option<integer>,
    /// The date/time of the Account’s last margin call extension.
    last_margin_call_extension_time: Option<DateTime>,
    /// The ID of the last Transaction created for the Account.
    last_transaction_id: Option<TransactionID>,
    /// The details of the Trades currently open in the Account.
    trades: Vec<TradeSummary>,
    /// The details all Account Positions.
    positions: Vec<Position>,
    /// The details of the Orders currently pending in the Account.
    orders: Vec<Order>,
}
