use chrono::DateTime;
use definitions::calculated_position_state::CalculatedPositionState;
use definitions::account_units::AccountUnits;
use definitions::calculated_trade_state::CalculatedTradeState;
use definitions::decimal_number::DecimalNumber;
use definitions::dynamic_order_state::DynamicOrderState;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct AccountChangesState {
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
    /// The price-dependent state of each pending Order in the
    /// Account.
    orders: Vec<DynamicOrderState>,
    /// The price-dependent state for each open Trade in the
    /// Account.
    trades: Vec<CalculatedTradeState>,
    /// The price-dependent state for each open Position in the
    /// Account.
    positions: Vec<CalculatedPositionState>,
}
impl Default for AccountChangesState {
    fn default() -> Self {
        use Default::default;
        Self {
            unrealized_pl: default(),
            nav: default(),
            margin_used: default(),
            margin_available: default(),
            position_value: default(),
            margin_closeout_unrealized_pl: default(),
            margin_closeout_nav: default(),
            margin_closeout_margin_used: default(),
            margin_closeout_percent: default(),
            margin_closeout_position_value: default(),
            withdrawal_limit: default(),
            margin_call_margin_used: default(),
            margin_call_percent: default(),
            balance: default(),
            pl: default(),
            resettable_pl: default(),
            financing: default(),
            commission: default(),
            dividend_adjustment: default(),
            guaranteed_execution_fees: default(),
            margin_call_enter_time: default(),
            margin_call_extension_count: default(),
            last_margin_call_extension_time: default(),
            orders: default(),
            trades: default(),
            positions: default(),
        }
    }
}
