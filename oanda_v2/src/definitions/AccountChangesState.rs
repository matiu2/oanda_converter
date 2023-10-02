use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct AccountChangesState {
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
    /// The current balance of the account.
    balance: Option<AccountUnits>,
    /// The total profit/loss realized over the lifetime of the Account.
    pl: Option<AccountUnits>,
    /// The total realized profit/loss for the account since it was last reset by the client.
    resettablePL: Option<AccountUnits>,
    /// The total amount of financing paid/collected over the lifetime of the account.
    financing: Option<AccountUnits>,
    /// The total amount of commission paid over the lifetime of the Account.
    commission: Option<AccountUnits>,
    /// The total amount of dividend adjustment paid over the lifetime of the Account in the Account’s home currency.
    dividendAdjustment: Option<AccountUnits>,
    /// The total amount of fees charged over the lifetime of the Account for the execution of guaranteed Stop Loss Orders.
    guaranteedExecutionFees: Option<AccountUnits>,
    /// The date/time when the Account entered a margin call state. Only provided if the Account is in a margin call.
    marginCallEnterTime: Option<DateTime>,
    /// The number of times that the Account’s current margin call was extended.
    marginCallExtensionCount: Option<integer>,
    /// The date/time of the Account’s last margin call extension.
    lastMarginCallExtensionTime: Option<DateTime>,
    /// The price-dependent state of each pending Order in the Account.
    orders: Vec<DynamicOrderState>,
    /// The price-dependent state for each open Trade in the Account.
    trades: Vec<CalculatedTradeState>,
    /// The price-dependent state for each open Position in the Account.
    positions: Vec<CalculatedPositionState>,
}
