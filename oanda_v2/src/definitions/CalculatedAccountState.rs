use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct CalculatedAccountState {
    /// The total unrealized profit/loss for all Trades currently open in the Account.
    unrealized_pl: Option<AccountUnits>,
    /// The net asset value of the Account. Equal to Account balance + unrealizedPL.
    nav: Option<AccountUnits>,
    /// Margin currently used for the Account.
    margin_used: Option<AccountUnits>,
    /// Margin available for Account currency.
    margin_available: Option<AccountUnits>,
    /// The value of the Account’s open positions represented in the Account’s home currency.
    position_value: Option<AccountUnits>,
    /// The Account’s margin closeout unrealized PL.
    margin_closeout_unrealized_pl: Option<AccountUnits>,
    /// The Account’s margin closeout NAV.
    margin_closeout_nav: Option<AccountUnits>,
    /// The Account’s margin closeout margin used.
    margin_closeout_margin_used: Option<AccountUnits>,
    /// The Account’s margin closeout percentage. When this value is 1.0 or above the Account is in a margin closeout situation.
    margin_closeout_percent: Option<DecimalNumber>,
    /// The value of the Account’s open positions as used for margin closeout calculations represented in the Account’s home currency.
    margin_closeout_position_value: Option<DecimalNumber>,
    /// The current WithdrawalLimit for the account which will be zero or a positive value indicating how much can be withdrawn from the account.
    withdrawal_limit: Option<AccountUnits>,
    /// The Account’s margin call margin used.
    margin_call_margin_used: Option<AccountUnits>,
    /// The Account’s margin call percentage. When this value is 1.0 or above the Account is in a margin call situation.
    margin_call_percent: Option<DecimalNumber>,
}
