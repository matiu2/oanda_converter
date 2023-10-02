use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct CalculatedAccountState {
    /// The total unrealized profit/loss for all Trades currently open in the Account.
    unrealizedPL: Option<AccountUnits>,
    /// The net asset value of the Account. Equal to Account balance + unrealizedPL.
    NAV: Option<AccountUnits>,
    /// Margin currently used for the Account.
    marginUsed: Option<AccountUnits>,
    /// Margin available for Account currency.
    marginAvailable: Option<AccountUnits>,
    /// The value of the Account’s open positions represented in the Account’s home currency.
    positionValue: Option<AccountUnits>,
    /// The Account’s margin closeout unrealized PL.
    marginCloseoutUnrealizedPL: Option<AccountUnits>,
    /// The Account’s margin closeout NAV.
    marginCloseoutNAV: Option<AccountUnits>,
    /// The Account’s margin closeout margin used.
    marginCloseoutMarginUsed: Option<AccountUnits>,
    /// The Account’s margin closeout percentage. When this value is 1.0 or above the Account is in a margin closeout situation.
    marginCloseoutPercent: Option<DecimalNumber>,
    /// The value of the Account’s open positions as used for margin closeout calculations represented in the Account’s home currency.
    marginCloseoutPositionValue: Option<DecimalNumber>,
    /// The current WithdrawalLimit for the account which will be zero or a positive value indicating how much can be withdrawn from the account.
    withdrawalLimit: Option<AccountUnits>,
    /// The Account’s margin call margin used.
    marginCallMarginUsed: Option<AccountUnits>,
    /// The Account’s margin call percentage. When this value is 1.0 or above the Account is in a margin call situation.
    marginCallPercent: Option<DecimalNumber>,
}
