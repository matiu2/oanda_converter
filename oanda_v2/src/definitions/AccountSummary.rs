use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct AccountSummary {
    /// The Account’s identifier
    id: Option<AccountID>,
    /// Client-assigned alias for the Account. Only provided if the Account has an alias set
    alias: Option<string>,
    /// The home currency of the Account
    currency: Option<Currency>,
    /// ID of the user that created the Account.
    createdByUserID: Option<integer>,
    /// The date/time when the Account was created.
    createdTime: Option<DateTime>,
    /// The current guaranteed Stop Loss Order settings of the Account. This field will only be present if the guaranteedStopLossOrderMode is not ‘DISABLED’.
    guaranteedStopLossOrderParameters: Option<GuaranteedStopLossOrderParameters>,
    /// The current guaranteed Stop Loss Order mode of the Account.
    guaranteedStopLossOrderMode: Option<GuaranteedStopLossOrderMode>,
    /// The date/time that the Account’s resettablePL was last reset.
    resettablePLTime: Option<DateTime>,
    /// Client-provided margin rate override for the Account. The effective margin rate of the Account is the lesser of this value and the OANDA margin rate for the Account’s division. This value is only provided if a margin rate override exists for the Account.
    marginRate: Option<DecimalNumber>,
    /// The number of Trades currently open in the Account.
    openTradeCount: Option<integer>,
    /// The number of Positions currently open in the Account.
    openPositionCount: Option<integer>,
    /// The number of Orders currently pending in the Account.
    pendingOrderCount: Option<integer>,
    /// Flag indicating that the Account has hedging enabled.
    hedgingEnabled: Option<boolean>,
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
    /// The ID of the last Transaction created for the Account.
    lastTransactionID: Option<TransactionID>,
}
