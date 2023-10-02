use serde::{Serialize, Deserialize};
_blank_!();
#[derive(Serialize, Deserialize)]
struct AccumulatedAccountState {
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
}
